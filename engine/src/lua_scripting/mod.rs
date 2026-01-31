//! Scripting system for DJ Engine.

use bevy::prelude::*;

pub mod context;
pub mod ffi;

pub use context::LuaContext;

#[derive(Resource, Clone, Default)]
pub struct SharedInputStateResource(pub ffi::SharedInputState);

#[derive(Message, Debug, Clone)]
pub enum ScriptCommand {
    Load { path: String },
    /// Execute a specific hook if it exists in the Lua environment
    Hook { name: String, args: Vec<String> },
}

pub struct DJScriptingPlugin;

impl Plugin for DJScriptingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LuaContext>()
            .init_resource::<SharedInputStateResource>()
            .add_message::<ScriptCommand>();
            
        // Register APIs on startup
        app.add_systems(Startup, setup_lua_environment)
           .add_systems(Update, (
               process_script_commands,
           ));
    }
}

fn process_script_commands(
    lua_ctx: Res<LuaContext>,
    mut commands: MessageReader<ScriptCommand>,
) {
    let mut script_cmds = commands.read().collect::<Vec<_>>();
    if script_cmds.is_empty() { return; }

    if let Ok(lua) = lua_ctx.lua.lock() {
        for cmd in script_cmds {
            match cmd {
                ScriptCommand::Load { path } => {
                    if let Ok(content) = std::fs::read_to_string(&path) {
                        info!("Loading Lua script: {}", path);
                        if let Err(e) = lua.load(&content).exec() {
                            error!("Error loading script {}: {}", path, e);
                        }
                    } else {
                        error!("Failed to read Lua script: {}", path);
                    }
                }
                ScriptCommand::Hook { name, args } => {
                    let globals = lua.globals();
                    if let Ok(func) = globals.get::<_, mlua::Function>(name.as_str()) {
                        let lua_args = mlua::Variadic(args.clone());
                        if let Err(e) = func.call::<_, ()>(lua_args) {
                            error!("Error calling Lua hook '{}': {}", name, e);
                        }
                    }
                }
            }
        }
    }
}

fn setup_lua_environment(
    lua_ctx: Res<LuaContext>,
    anim_cmds: Option<Res<crate::animation::SharedAnimationCommands>>,
    input_state: Option<Res<SharedInputStateResource>>,
) {
    if let Ok(lua) = lua_ctx.lua.lock() {
        // 1. Core logger APIs
        if let Err(e) = ffi::register_core_api(&lua) {
            error!("Failed to register core Lua API: {}", e);
        }
        
        // 2. Animation APIs
        if let Some(cmds) = anim_cmds {
            if let Err(e) = ffi::register_animation_api(&lua, cmds.clone()) {
                error!("Failed to register animation Lua API: {}", e);
            }
        }

        // 3. Input APIs
        if let Some(input) = input_state {
            if let Err(e) = ffi::register_input_api(&lua, input.0.clone()) {
                error!("Failed to register input Lua API: {}", e);
            }
        }

        // 4. Story APIs
        if let Err(e) = ffi::register_story_api(&lua) {
            error!("Failed to register story Lua API: {}", e);
        }

        // 5. Generic State (for global bridge)
        let state = ffi::create_shared_state();
        if let Err(e) = ffi::register_generic_state_api(&lua, state) {
            error!("Failed to register generic state Lua API: {}", e);
        }
    }
}
