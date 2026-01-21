//! Lua context management for DJ Engine.
//!
//! Handles initialization of the Lua VM and global state.

use bevy::prelude::*;
use mlua::prelude::*;
use std::sync::Mutex;

/// Resource storing the Lua interpreter instance.
/// Uses a Mutex because mlua::Lua is not Sync, but we need it in Bevy resources.
/// We usually access this only from the main thread or specific systems.
#[derive(Resource)]
pub struct LuaContext {
    pub lua: Mutex<Lua>,
}

impl Default for LuaContext {
    fn default() -> Self {
        Self::new()
    }
}

impl LuaContext {
    /// Creates a new Lua context with standard libraries.
    pub fn new() -> Self {
        let lua = Lua::new();
        // Sandbox controls could go here (e.g., restricting IO)
        Self {
            lua: Mutex::new(lua),
        }
    }
}
