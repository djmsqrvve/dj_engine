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
}

pub struct DJScriptingPlugin;

impl Plugin for DJScriptingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LuaContext>()
            .init_resource::<SharedInputStateResource>()
            .add_message::<ScriptCommand>();
            
        // Register APIs on startup
        app.add_systems(Startup, setup_lua_environment);
    }
}

fn setup_lua_environment(
    lua_ctx: Res<LuaContext>,
    anim_cmds: Option<Res<crate::animation::SharedAnimationCommands>>,
) {
    if let Ok(lua) = lua_ctx.lua.lock() {
        if let Err(e) = ffi::register_core_api(&lua) {
            error!("Failed to register core Lua API: {}", e);
        }
        
        if let Some(cmds) = anim_cmds {
            if let Err(e) = ffi::register_animation_api(&lua, cmds.clone()) {
                error!("Failed to register animation Lua API: {}", e);
            }
        }
    }
}
