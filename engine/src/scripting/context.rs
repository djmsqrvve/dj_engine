//! Lua context management for DJ Engine.
//!
//! Handles initialization of the Lua VM and global state.

use bevy::prelude::*;
use mlua::prelude::*;
use mlua::{LuaOptions, StdLib};
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
    /// Creates a new sandboxed Lua context with restricted libraries.
    pub fn new() -> Self {
        // Only load safe libraries. Exclude IO, OS, and Debug for security.
        let lua = Lua::new_with(
            StdLib::TABLE | StdLib::STRING | StdLib::MATH | StdLib::UTF8,
            LuaOptions::default(),
        )
        .expect("Failed to initialize sandboxed Lua VM");

        Self {
            lua: Mutex::new(lua),
        }
    }
}
