//! Game mode data structure.
//!
//! A [`GameMode`] defines the ruleset and logic for a game session.
//! It contains script references and global constants.

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A game mode configuration (e.g., "Survival", "Story", "PVP").
#[derive(Debug, Clone, Serialize, Deserialize, Reflect)]
pub struct GameMode {
    /// Unique mode ID
    pub id: String,
    /// Display name
    pub name: String,

    /// Path to the main Lua script for this mode
    /// e.g. "scripts/modes/survival.lua"
    pub script_path: String,

    /// Entry point function name in the script
    #[serde(default = "default_entry_point")]
    pub entry_point: String,

    /// Global tuning constants exposed to Lua
    #[serde(default)]
    pub constants: HashMap<String, f32>,

    /// Required player count (min/max)
    #[serde(default)]
    pub player_count: PlayerCountRange,
}

fn default_entry_point() -> String {
    "init_mode".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize, Reflect)]
pub struct PlayerCountRange {
    pub min: u32,
    pub max: u32,
}

impl Default for PlayerCountRange {
    fn default() -> Self {
        Self { min: 1, max: 1 }
    }
}

impl Default for GameMode {
    fn default() -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name: "New Mode".to_string(),
            script_path: String::new(),
            entry_point: default_entry_point(),
            constants: HashMap::new(),
            player_count: PlayerCountRange::default(),
        }
    }
}
