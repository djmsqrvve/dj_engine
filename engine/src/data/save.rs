use crate::story_graph::types::{StoryFlags, GraphExecutor, NodeId, FlagValue};
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

pub struct SavePlugin;

impl Plugin for SavePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SaveEvent>()
            .add_event::<LoadEvent>()
            .add_systems(Update, (handle_save_event, handle_load_event));
    }
}

#[derive(Event)]
pub struct SaveEvent(pub String); // Slot name or path

#[derive(Event)]
pub struct LoadEvent(pub String);

#[derive(Serialize, Deserialize, Debug)]
pub struct GameSaveData {
    pub format_version: String,
    pub timestamp: f64,
    pub story_flags: HashMap<String, FlagValue>,
    pub current_node: Option<NodeId>,
    pub player_position: Vec3,
    // TODO(#104): Add Inventory and Quests to save data
}

fn handle_save_event(
    mut events: EventReader<SaveEvent>,
    flags: Res<StoryFlags>,
    executor: Res<GraphExecutor>,
    player_query: Query<&Transform, With<crate::editor::state::LogicalEntity>>,
) {
    for event in events.read() {
        let player_pos = player_query.get_single().map(|t| t.translation).unwrap_or(Vec3::ZERO);
        
        let save_data = GameSaveData {
            format_version: "1.0".to_string(),
            timestamp: 0.0, // Should use real time
            story_flags: flags.0.clone(),
            current_node: executor.current_node,
            player_position: player_pos,
        };

        match serde_json::to_string_pretty(&save_data) {
            Ok(json) => {
                let path = format!("{}.json", event.0);
                if let Err(e) = fs::write(&path, json) {
                    error!("Failed to write save file {}: {}", path, e);
                } else {
                    info!("Game saved to {}", path);
                }
            }
            Err(e) => error!("Failed to serialize save data: {}", e),
        }
    }
}

fn handle_load_event(
    mut events: EventReader<LoadEvent>,
    mut flags: ResMut<StoryFlags>,
    mut executor: ResMut<GraphExecutor>,
    mut player_query: Query<&mut Transform, With<crate::editor::state::LogicalEntity>>,
) {
    for event in events.read() {
        let path = format!("{}.json", event.0);
        match fs::read_to_string(&path) {
            Ok(json) => {
                match serde_json::from_str::<GameSaveData>(&json) {
                    Ok(data) => {
                        *flags = StoryFlags(data.story_flags);
                        executor.current_node = data.current_node;
                        // Force executor to resume if it was running/waiting
                        if executor.active_graph_id.is_some() {
                           executor.status = crate::story_graph::types::ExecutionStatus::Running;
                        }
                        
                        if let Ok(mut transform) = player_query.get_single_mut() {
                            transform.translation = data.player_position;
                        }
                        
                        info!("Game loaded from {}", path);
                    }
                    Err(e) => error!("Failed to deserialize save file {}: {}", path, e),
                }
            }
            Err(e) => error!("Failed to read save file {}: {}", path, e),
        }
    }
}
