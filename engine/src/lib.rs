//! DJ Engine - Core shared library for cursed narrative games
//!
//! This crate provides the foundational systems for building procedural
//! 2D character animation games with Lua scripting and palette-driven effects.
//!
//! # Example
//!
//! ```ignore
//! use dj_engine::prelude::*;
//!
//! App::new()
//!     .add_plugins(DefaultPlugins)
//!     .add_plugins(DJEnginePlugin::default())
//!     .run();
//! ```

pub mod assets;
pub mod core;
pub mod data;
pub mod diagnostics;
pub mod story_graph;
pub mod types;

/// Prelude module for convenient imports
pub mod prelude {
    pub use crate::core::DJEnginePlugin;
    pub use crate::assets::DJAssetPlugin;
    pub use crate::diagnostics::DiagnosticsPlugin;
    pub use crate::story_graph::*;
    pub use crate::types::*;
    pub use crate::data::{
        load_database, load_project, load_scene, load_story_graph, AssetIndex, CampaignData,
        CampaignNodeData, CampaignNodeType, DataError, Database, EditorPreferences, EnemyRow,
        Entity, EntityType, ItemRow, Layer, LootTableRow, NpcRow, Prefab, Project, ProjectSettings,
        QuestRow, Scene, SceneType, StoryGraphData, StoryNodeData, StoryNodeType, TowerRow,
    };
}

/// Returns the current engine version from Cargo.toml
pub fn engine_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}
