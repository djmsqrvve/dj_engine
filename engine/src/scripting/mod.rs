//! Scripting system for DJ Engine.
//!
//! Provides Lua integration via mlua. Games extend this with their own APIs.

use bevy::prelude::*;

pub mod context;
pub mod ffi;

use crate::story_graph::{executor::StoryActionEvent, StoryFlowEvent, StoryInputEvent};
pub use context::LuaContext;
pub use ffi::{
    create_shared_input_state, create_shared_state, register_core_api, register_generic_state_api,
    register_input_api, register_story_api, GenericStateBuffer, SharedGenericState,
    SharedInputState,
};

/// Events for script control.
#[derive(Event, Debug, Clone, Reflect)]
pub enum ScriptCommand {
    /// Load and execute a Lua script from file
    Load { path: String },
}

/// Scripting plugin that provides the Lua runtime.
pub struct DJScriptingPlugin;

impl Plugin for DJScriptingPlugin {
    fn build(&self, app: &mut App) {
        let lua_ctx = LuaContext::new();
        let input_state = create_shared_input_state();
        
        // Get shared animation commands if available (assuming DJAnimationPlugin is present)
        let animation_commands = app.world().get_resource::<crate::animation::SharedAnimationCommands>().cloned();

        // Register core APIs (log, warn, error)
        {
            match lua_ctx.lua.lock() {
                Ok(lua) => {
                    if let Err(e) = ffi::register_core_api(&lua) {
                        error!("Failed to register core Lua API: {}", e);
                    }
                    if let Err(e) = ffi::register_story_api(&lua) {
                        error!("Failed to register Story Lua API: {}", e);
                    }
                    if let Err(e) = ffi::register_input_api(&lua, input_state.clone()) {
                        error!("Failed to register Input Lua API: {}", e);
                    }
                    if let Some(anim_cmds) = &animation_commands {
                        if let Err(e) = ffi::register_animation_api(&lua, anim_cmds.clone()) {
                            error!("Failed to register Animation Lua API: {}", e);
                        }
                    }
                }
                Err(e) => {
                    error!("Failed to acquire Lua lock during initialization: {}", e);
                }
            }
        }

        app.insert_resource(lua_ctx)
            .insert_resource(SharedInputStateResource(input_state))
            .register_type::<ScriptCommand>()
            .add_event::<ScriptCommand>()
            .add_systems(Update, (handle_script_commands, bridge_story_events_to_lua, sync_input_to_lua));

        info!("DJ Scripting Plugin initialized");
    }
}

/// Resource wrapper for shared input state.
#[derive(Resource)]
pub struct SharedInputStateResource(pub SharedInputState);

fn sync_input_to_lua(
    action_state: Res<crate::input::ActionState>,
    shared_input: Res<SharedInputStateResource>,
) {
    if let Ok(mut buffer) = shared_input.0.write() {
        buffer.cursor_position = action_state.cursor_position;
        
        buffer.pressed.clear();
        for action in action_state.iter_pressed() {
            buffer.pressed.insert(action.as_str().to_string());
        }
        
        buffer.just_pressed.clear();
        for action in action_state.iter_just_pressed() {
            buffer.just_pressed.insert(action.as_str().to_string());
        }
        
        buffer.just_released.clear();
        for action in action_state.iter_just_released() {
            buffer.just_released.insert(action.as_str().to_string());
        }
    }
}

/// System that bridges Bevy StoryFlowEvents to Lua global callbacks.
fn bridge_story_events_to_lua(
    lua_ctx: Res<LuaContext>,
    mut flow_events: EventReader<StoryFlowEvent>,
    mut action_events: EventReader<StoryActionEvent>,
    mut story_input: EventWriter<StoryInputEvent>,
) {
    // Handle incoming from Lua
    if crate::scripting::ffi::STORY_ADVANCE_PENDING.swap(false, std::sync::atomic::Ordering::SeqCst)
    {
        story_input.send(StoryInputEvent::Advance);
    }

    let lua_guard = match lua_ctx.lua.lock() {
        Ok(guard) => guard,
        Err(poisoned) => {
            error!("Lua mutex poisoned: {}", poisoned);
            return;
        }
    };

    for event in flow_events.read() {
        if let StoryFlowEvent::ShowDialogue {
                speaker,
                text,
                portrait: _,
            } = event {
            let globals = lua_guard.globals();
            if let Ok(func) = globals.get::<mlua::Function>("on_dialogue") {
                if let Err(e) = func.call::<()>((speaker.clone(), text.clone())) {
                    error!("Lua on_dialogue error: {}", e);
                }
            }
        }
    }

    for event in action_events.read() {
        let globals = lua_guard.globals();
        if let Ok(func) = globals.get::<mlua::Function>("on_story_action") {
            if let Err(e) = func.call::<()>((event.script_id.clone(), event.params.to_string())) {
                error!("Lua on_story_action error: {}", e);
            }
        }
    }
}

/// System that processes script commands.
fn handle_script_commands(lua_ctx: Res<LuaContext>, mut events: EventReader<ScriptCommand>) {
    for event in events.read() {
        match event {
            ScriptCommand::Load { path } => {
                info!("Scripting: Loading script from {}", path);

                // 1. Path Sanitization
                let base_dir = std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
                let script_path = match sanitize_path(&base_dir, path) {
                    Ok(p) => p,
                    Err(e) => {
                        error!("Scripting: Sandbox violation or invalid path: {}", e);
                        continue;
                    }
                };

                if !script_path.exists() {
                    error!("Script not found: {:?}", script_path);
                    continue;
                }

                let lua_guard = match lua_ctx.lua.lock() {
                    Ok(guard) => guard,
                    Err(poisoned) => {
                        error!("Lua mutex poisoned during load: {}", poisoned);
                        return;
                    }
                };

                    // 2. Execution Limits & Panic Safety
                let result: mlua::Result<()> = (|| {
                    let script = std::fs::read_to_string(&script_path)?;
                    
                    // Set an instruction limit to prevent infinite loops (e.g., 100,000 instructions)
                    let instructions = std::sync::Arc::new(std::sync::atomic::AtomicU32::new(0));
                    let instr_handle = instructions.clone();
                    lua_guard.set_hook(mlua::HookTriggers::default().every_nth_instruction(1000), move |_, _| {
                        let current = instr_handle.fetch_add(1000, std::sync::atomic::Ordering::SeqCst);
                        if current + 1000 > 100_000 {
                            return Err(mlua::Error::RuntimeError("Script instruction limit exceeded".to_string()));
                        }
                        Ok(mlua::VmState::Continue)
                    });

                    // Execute script
                    let exec_result = lua_guard.load(&script).exec();
                    
                    // Remove hook after execution
                    lua_guard.remove_hook();
                    exec_result
                })();

                if let Err(e) = result {
                    error!("Failed to execute script {:?}: {}", script_path, e);
                }
            }
        }
    }
}

/// Errors that can occur during path sanitization.
#[derive(Debug, thiserror::Error)]
enum PathSanitizationError {
    #[error("Failed to canonicalize base directory: {0}")]
    CanonicalizeBase(std::io::Error),
    #[error("Failed to canonicalize script path: {0}")]
    CanonicalizePath(std::io::Error),
    #[error("Directory traversal attempt detected: path escapes sandbox")]
    DirectoryTraversal,
}

/// Helper to ensure a path stays within a base directory (Sandbox).
fn sanitize_path(base: &std::path::Path, tail: &str) -> Result<std::path::PathBuf, PathSanitizationError> {
    let path = base.join(tail);
    let canonical_base = base.canonicalize().map_err(PathSanitizationError::CanonicalizeBase)?;
    
    // Attempt to canonicalize the full path. If it doesn't exist yet, we can't fully canonicalize,
    // but for LOADING a script it MUST exist.
    let canonical_path = path.canonicalize().map_err(PathSanitizationError::CanonicalizePath)?;

    if canonical_path.starts_with(&canonical_base) {
        Ok(canonical_path)
    } else {
        Err(PathSanitizationError::DirectoryTraversal)
    }
}

#[cfg(test)]
mod tests;
