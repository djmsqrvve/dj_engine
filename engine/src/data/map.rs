//! Map data structures for static environments.
//!
//! A [`MapAsset`] represents the static geometry and terrain of a world.
//! It contains tile layers, navigation meshes, and static physical properties,
//! but NO dynamic entities or game logic.

use super::components::ColorData;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// A complete static map.
#[derive(Debug, Clone, Serialize, Deserialize, Reflect)]
pub struct MapAsset {
    /// Unique map identifier
    pub id: String,
    /// Human-readable map name
    pub name: String,

    /// Map dimensions in tiles
    pub size: MapSize,
    /// Visual background color or texture
    #[serde(default)]
    pub background: ColorData,

    /// Layers of static tiles (rendering order: 0=bottom)
    #[serde(default)]
    pub layers: Vec<MapLayer>,

    /// Navigation mesh for pathfinding
    #[serde(default)]
    pub nav_mesh: NavGrid,

    /// Physics collision data (static walls/colliders)
    #[serde(default)]
    pub physics: MapPhysics,
}

impl Default for MapAsset {
    fn default() -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name: "New Map".to_string(),
            size: MapSize {
                width: 32,
                height: 32,
            },
            background: ColorData::black(),
            layers: vec![MapLayer::new("ground", "Ground").with_order(0)],
            nav_mesh: NavGrid::default(),
            physics: MapPhysics::default(),
        }
    }
}

/// Map dimensions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize, Reflect)]
pub struct MapSize {
    pub width: u32,
    pub height: u32,
}

/// A single layer of tiles in the map.
#[derive(Debug, Clone, Serialize, Deserialize, Reflect)]
pub struct MapLayer {
    /// Unique layer ID
    pub id: String,
    /// Display name
    pub name: String,
    /// Visible in editor/game
    #[serde(default = "default_true")]
    pub visible: bool,
    /// Locked in editor
    #[serde(default)]
    pub locked: bool,
    /// Rendering order
    pub order: i32,
    /// Flattened tile data (row-major: y * width + x)
    /// 0 = empty/air
    pub tiles: Vec<u32>,
}

impl MapLayer {
    pub fn new(id: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            visible: true,
            locked: false,
            order: 0,
            tiles: Vec::new(),
        }
    }

    pub fn with_order(mut self, order: i32) -> Self {
        self.order = order;
        self
    }
}

fn default_true() -> bool {
    true
}

/// Navigation grid data.
#[derive(Debug, Clone, Default, Serialize, Deserialize, Reflect)]
pub struct NavGrid {
    /// Grid width
    pub width: u32,
    /// Grid height
    pub height: u32,
    /// Walkability mask (true = walkable)
    #[serde(default)]
    pub walkable: Vec<bool>,
    /// Movement cost multiplier (default 1.0)
    #[serde(default)]
    pub costs: Vec<f32>,
}

/// Static physics definition for the map.
#[derive(Debug, Clone, Default, Serialize, Deserialize, Reflect)]
pub struct MapPhysics {
    /// List of static collision boxes (e.g., walls)
    #[serde(default)]
    pub colliders: Vec<MapCollider>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Reflect)]
pub struct MapCollider {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    #[serde(default)]
    pub layer: String, // e.g. "wall", "water"
}
