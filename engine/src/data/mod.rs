//! Data model types for dj_engine editor and runtime.
//!
//! This module contains serializable data structures used for:
//! - Project configuration and settings
//! - Scene composition (layers, entities, components)
//! - Story graphs and dialogue systems
//! - Game databases (items, NPCs, towers, enemies, etc.)
//! - Asset indexing and prefabs
//!
//! These types are designed to be stored as JSON and loaded by both
//! the editor and runtime. They are intentionally separate from Bevy
//! ECS components to maintain a clean data transfer layer.

pub mod project;
pub mod scene;
pub mod components;
pub mod story;
pub mod database;
pub mod assets;
pub mod loader;
pub mod spawner;

// Re-export commonly used types
pub use project::{Project, ProjectSettings, EditorPreferences};
pub use scene::{Scene, Layer, Entity, SceneType, EntityType};
pub use components::*;
pub use story::{StoryGraphData, StoryNodeData, StoryNodeType};
pub use database::{Database, ItemRow, NpcRow, TowerRow, EnemyRow, LootTableRow, QuestRow};
pub use assets::{AssetIndex, Prefab};
pub use loader::{load_project, load_scene, load_database, load_story_graph, DataError};
