//! Scripting system for DJ Engine.
//!
//! Provides Lua integration via mlua. Games extend this with their own APIs.

use bevy::prelude::*;

pub mod context;
pub mod ffi;

pub use context::LuaContext;
pub use ffi::{
    create_shared_state, register_core_api, register_generic_state_api, GenericStateBuffer,
    SharedGenericState,
};
use crate::story_graph::{StoryFlowEvent, StoryInputEvent, executor::StoryActionEvent};

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

        // Register core APIs (log, warn, error)
        {
            let lua = lua_ctx.lua.lock().unwrap();
            if let Err(e) = ffi::register_core_api(&lua) {
                error!("Failed to register core Lua API: {}", e);
            }
            if let Err(e) = ffi::register_story_api(&lua) {
                error!("Failed to register Story Lua API: {}", e);
            }
        }

        app.insert_resource(lua_ctx)
            .register_type::<ScriptCommand>()
            .add_event::<ScriptCommand>()
            .add_systems(Update, (
                handle_script_commands,
                bridge_story_events_to_lua,
            ));

        info!("DJ Scripting Plugin initialized");
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
    if crate::scripting::ffi::STORY_ADVANCE_PENDING.swap(false, std::sync::atomic::Ordering::SeqCst) {
        story_input.send(StoryInputEvent::Advance);
    }

    let lua = lua_ctx.lua.lock().unwrap();
    
    for event in flow_events.read() {
        match event {
            StoryFlowEvent::ShowDialogue { speaker, text, portrait: _ } => {
                let globals = lua.globals();
                if let Ok(func) = globals.get::<mlua::Function>("on_dialogue") {
                    if let Err(e) = func.call::<()>( (speaker.clone(), text.clone()) ) {
                        error!("Lua on_dialogue error: {}", e);
                    }
                }
            }
            _ => {}
        }
    }

    for event in action_events.read() {
        let globals = lua.globals();
        if let Ok(func) = globals.get::<mlua::Function>("on_story_action") {
            if let Err(e) = func.call::<()>( (event.script_id.clone(), event.params.to_string()) ) {
                error!("Lua on_story_action error: {}", e);
            }
        }
    }
}

/// System that processes script commands.
fn handle_script_commands(
    lua_ctx: Res<LuaContext>,
    mut events: EventReader<ScriptCommand>,
) {
    for event in events.read() {
        match event {
            ScriptCommand::Load { path } => {
                info!("Scripting: Loading script from {}", path);
                let lua = lua_ctx.lua.lock().unwrap();
                
                let result: mlua::Result<()> = (|| {
                    let script = std::fs::read_to_string(path)?;
                    lua.load(&script).exec()
                })();

                if let Err(e) = result {
                    error!("Failed to execute script {}: {}", path, e);
                }
            }
        }
    }
}
