//! Scenario data structures for missions and levels.
//!
//! A [`ScenarioData`] binds a static [`MapAsset`] with a [`GameMode`] and dynamic entities.
//! It represents a playable mission or level instance.

use super::components::{EntityComponents, Vec3Data};
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// A playable scenario (Map + Mode + Entities).
#[derive(Debug, Clone, Serialize, Deserialize, Reflect)]
pub struct ScenarioData {
    /// Unique scenario ID
    pub id: String,
    /// Display name
    pub name: String,

    /// ID of the MapAsset to use
    pub map_id: String,

    /// ID of the GameMode to use
    pub mode_id: String,

    /// Dynamic entities (units, spawners) specific to this scenario
    #[serde(default)]
    pub entities: Vec<ScenarioEntity>,

    /// Specific objective/logic script for this scenario
    /// (overrides or extends mode script)
    #[serde(default)]
    pub script_path: Option<String>,

    /// Scenario-specific configuration overrides
    #[serde(default)]
    pub config_overrides: std::collections::HashMap<String, f32>,
}

impl Default for ScenarioData {
    fn default() -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name: "New Scenario".to_string(),
            map_id: String::new(),
            mode_id: String::new(),
            entities: Vec::new(),
            script_path: None,
            config_overrides: std::collections::HashMap::new(),
        }
    }
}

/// A dynamic entity instance in a scenario.
#[derive(Debug, Clone, Serialize, Deserialize, Reflect)]
pub struct ScenarioEntity {
    /// Unique instance ID
    pub id: String,
    /// Optional Name for logic/debugging
    #[serde(default)]
    pub name: String,

    /// Prefab ID if this spawns from a template
    #[serde(default)]
    pub prefab_id: Option<String>,

    /// Initial spawn position
    pub position: Vec3Data,

    /// Rotation (degrees)
    #[serde(default)]
    pub rotation: Vec3Data,

    /// Component overrides (merged with prefab if exists)
    #[serde(default)]
    pub components: EntityComponents,
}

impl Default for ScenarioEntity {
    fn default() -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name: "Entity".to_string(),
            prefab_id: None,
            position: Vec3Data::default(),
            rotation: Vec3Data::default(),
            components: EntityComponents::default(),
        }
    }
}
