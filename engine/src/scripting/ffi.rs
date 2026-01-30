//! FFI bindings between Rust and Lua.
//!
//! Provides a generic scripting bridge that games can extend with their own APIs.

use bevy::prelude::*;
use mlua::prelude::*;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, RwLock};

/// Registers core FFI functions into the Lua global table.
/// Games should call this first, then register their own APIs.
pub fn register_core_api(lua: &Lua) -> LuaResult<()> {
    let globals = lua.globals();

    // log(string)
    let log_fn = lua.create_function(|_, message: String| {
        info!("[Lua] {}", message);
        Ok(())
    })?;
    globals.set("log", log_fn)?;

    // warn(string)
    let warn_fn = lua.create_function(|_, message: String| {
        warn!("[Lua] {}", message);
        Ok(())
    })?;
    globals.set("warn", warn_fn)?;

    // error(string)
    let error_fn = lua.create_function(|_, message: String| {
        error!("[Lua] {}", message);
        Ok(())
    })?;
    globals.set("error", error_fn)?;

    Ok(())
}

/// Generic shared state buffer for bridging Lua (non-ECS) and Bevy (ECS).
/// Games should create their own state buffer types.
#[derive(Default, Debug)]
pub struct GenericStateBuffer {
    /// Key-value store for simple state
    pub floats: std::collections::HashMap<String, f32>,
    pub strings: std::collections::HashMap<String, String>,
    pub bools: std::collections::HashMap<String, bool>,
}

pub type SharedGenericState = Arc<RwLock<GenericStateBuffer>>;

/// Helper to create a shared generic state buffer.
pub fn create_shared_state() -> SharedGenericState {
    Arc::new(RwLock::new(GenericStateBuffer::default()))
}

/// Maximum string length for Lua keys/values (1MB to prevent accidental memory exhaustion).
const MAX_LUA_STRING_LEN: usize = 1_048_576;

/// Helper function to handle RwLock poisoning.
/// Panics on poisoned locks since this indicates a thread panic and is unrecoverable.
fn lock_state<'a, T>(result: std::result::Result<std::sync::RwLockReadGuard<'a, T>, std::sync::PoisonError<std::sync::RwLockReadGuard<'a, T>>>) -> LuaResult<std::sync::RwLockReadGuard<'a, T>> {
    result.map_err(|_| LuaError::RuntimeError("State lock poisoned - thread panic detected".into()))
}

fn lock_state_mut<'a, T>(result: std::result::Result<std::sync::RwLockWriteGuard<'a, T>, std::sync::PoisonError<std::sync::RwLockWriteGuard<'a, T>>>) -> LuaResult<std::sync::RwLockWriteGuard<'a, T>> {
    result.map_err(|_| LuaError::RuntimeError("State lock poisoned - thread panic detected".into()))
}

/// Register generic state access functions.
pub fn register_generic_state_api(lua: &Lua, state: SharedGenericState) -> LuaResult<()> {
    let globals = lua.globals();

    // set_float(key, value)
    let set_float = lua.create_function(move |_, (key, value): (String, f32)| {
        if key.len() > MAX_LUA_STRING_LEN {
            return Err(LuaError::RuntimeError(format!("Key too long (max {} bytes)", MAX_LUA_STRING_LEN)));
        }
        let mut data = lock_state_mut(Arc::clone(&state).write())?;
        data.floats.insert(key, value);
        Ok(())
    })?;
    globals.set("set_float", set_float)?;

    // get_float(key) -> f32
    let get_float = lua.create_function(move |_, key: String| {
        let data = lock_state(Arc::clone(&state).read())?;
        Ok(data.floats.get(&key).copied().unwrap_or(0.0))
    })?;
    globals.set("get_float", get_float)?;

    // set_string(key, value)
    let set_string = lua.create_function(move |_, (key, value): (String, String)| {
        if key.len() > MAX_LUA_STRING_LEN || value.len() > MAX_LUA_STRING_LEN {
            return Err(LuaError::RuntimeError(format!("String too long (max {} bytes)", MAX_LUA_STRING_LEN)));
        }
        let mut data = lock_state_mut(Arc::clone(&state).write())?;
        data.strings.insert(key, value);
        Ok(())
    })?;
    globals.set("set_string", set_string)?;

    // get_string(key) -> string
    let get_string = lua.create_function(move |_, key: String| {
        let data = lock_state(Arc::clone(&state).read())?;
        Ok(data.strings.get(&key).cloned().unwrap_or_default())
    })?;
    globals.set("get_string", get_string)?;

    // set_bool(key, value)
    let set_bool = lua.create_function(move |_, (key, value): (String, bool)| {
        if key.len() > MAX_LUA_STRING_LEN {
            return Err(LuaError::RuntimeError(format!("Key too long (max {} bytes)", MAX_LUA_STRING_LEN)));
        }
        let mut data = lock_state_mut(Arc::clone(&state).write())?;
        data.bools.insert(key, value);
        Ok(())
    })?;
    globals.set("set_bool", set_bool)?;

    // get_bool(key) -> bool
    let get_bool = lua.create_function(move |_, key: String| {
        let data = lock_state(Arc::clone(&state).read())?;
        Ok(data.bools.get(&key).copied().unwrap_or(false))
    })?;
    globals.set("get_bool", get_bool)?;

    Ok(())
}

/// Shared input state buffer for bridging ActionState (ECS) to Lua (non-ECS).
#[derive(Default, Debug)]
pub struct InputStateBuffer {
    pub pressed: std::collections::HashSet<String>,
    pub just_pressed: std::collections::HashSet<String>,
    pub just_released: std::collections::HashSet<String>,
    pub cursor_position: Vec2,
}

pub type SharedInputState = Arc<RwLock<InputStateBuffer>>;

pub fn create_shared_input_state() -> SharedInputState {
    Arc::new(RwLock::new(InputStateBuffer::default()))
}

/// Register input state access functions.
pub fn register_input_api(lua: &Lua, state: SharedInputState) -> LuaResult<()> {
    let globals = lua.globals();
    let actions = lua.create_table()?;

    // actions.pressed(action_name) -> bool
    let pressed = lua.create_function(move |_, action: String| {
        let data = lock_state(Arc::clone(&state).read())?;
        Ok(data.pressed.contains(&action))
    })?;
    actions.set("pressed", pressed)?;

    // actions.just_pressed(action_name) -> bool
    let just_pressed = lua.create_function(move |_, action: String| {
        let data = lock_state(Arc::clone(&state).read())?;
        Ok(data.just_pressed.contains(&action))
    })?;
    actions.set("just_pressed", just_pressed)?;

    // actions.just_released(action_name) -> bool
    let just_released = lua.create_function(move |_, action: String| {
        let data = lock_state(Arc::clone(&state).read())?;
        Ok(data.just_released.contains(&action))
    })?;
    actions.set("just_released", just_released)?;

    // actions.mouse_pos() -> (x, y)
    let mouse_pos = lua.create_function(move |_, ()| {
        let data = lock_state(Arc::clone(&state).read())?;
        Ok((data.cursor_position.x, data.cursor_position.y))
    })?;
    actions.set("mouse_pos", mouse_pos)?;

    globals.set("actions", actions)?;

    Ok(())
}

pub static STORY_ADVANCE_PENDING: AtomicBool = AtomicBool::new(false);

/// Register Story Graph control APIs.
pub fn register_story_api(lua: &Lua) -> LuaResult<()> {
    let globals = lua.globals();

    // next_node() -> Advance the story graph
    let next_node = lua.create_function(|_, ()| {
        STORY_ADVANCE_PENDING.store(true, Ordering::SeqCst);
        Ok(())
    })?;

    globals.set("story_next", next_node)?;

    Ok(())
}

/// Register Animation APIs.
pub fn register_animation_api(lua: &Lua, shared: crate::animation::SharedAnimationCommands) -> LuaResult<()> {
    let globals = lua.globals();
    let animation = lua.create_table()?;

    // animation.set_expression(id, expression)
    let set_expression = lua.create_function(move |_, (id, expr): (String, String)| {
        if let Ok(mut queue) = shared.0.lock() {
            queue.push(crate::animation::AnimationCommand::SetExpression {
                target_id: id,
                expression: expr,
            });
        }
        Ok(())
    })?;
    animation.set("set_expression", set_expression)?;

    globals.set("animation", animation)?;

    Ok(())
}
