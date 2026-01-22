//! Campaign data structures for high-level game flow.
//!
//! A Campaign organizes the game into Acts and Chapters, linking
//! individual Story Graphs and Scenes into a cohesive progression.

use serde::{Deserialize, Serialize};
use bevy::prelude::*;

use super::components::Vec3Data;

/// Type of campaign node.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize, Reflect)]
#[serde(rename_all = "snake_case")]
pub enum CampaignNodeType {
    #[default]
    Start,
    /// A generic story graph (dialogue, cutscene, etc.)
    StoryGraph,
    /// A gameplay scene (combat, exploration)
    Scene,
    /// A visual grouping node (Chapter/Act marker)
    Act,
    /// End of the campaign
    End,
}

/// A node in the campaign graph.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Reflect)]
pub struct CampaignNodeData {
    /// Unique ID
    pub id: String,
    /// Display name (e.g., "Chapter 1: The Beginning")
    pub name: String,
    /// Type of node
    #[serde(default)]
    pub node_type: CampaignNodeType,
    /// Position in the editor canvas
    #[serde(default)]
    pub position: Vec3Data,
    /// Reference to the content file (story graph path or scene path)
    #[serde(default)]
    pub content_path: Option<String>,
    /// IDs of nodes this connects to
    #[serde(default)]
    pub next_node_ids: Vec<String>,
    /// Description for the storyboard
    #[serde(default)]
    pub description: String,
}

impl Default for CampaignNodeData {
    fn default() -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name: "New Node".to_string(),
            node_type: CampaignNodeType::default(),
            position: Vec3Data::default(),
            content_path: None,
            next_node_ids: Vec::new(),
            description: String::new(),
        }
    }
}

/// The high-level Campaign container.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize, Reflect)]
pub struct CampaignData {
    /// Unique ID
    pub id: String,
    /// Campaign Title
    pub title: String,
    /// Root node ID (entry point)
    pub root_node_id: String,
    /// All nodes (Acts, Scenes, Graphs)
    pub nodes: Vec<CampaignNodeData>,
}

impl CampaignData {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            title: title.into(),
            root_node_id: String::new(),
            nodes: Vec::new(),
        }
    }

    pub fn add_node(&mut self, node: CampaignNodeData) {
        self.nodes.push(node);
    }
}
