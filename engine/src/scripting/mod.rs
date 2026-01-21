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

/// Scripting plugin that provides the Lua runtime.
/// Note: This plugin only sets up the Lua context and core APIs.
/// Games should register their own APIs and sync systems.
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
        }

        app.insert_resource(lua_ctx);

        info!("DJ Scripting Plugin initialized");
    }
}
