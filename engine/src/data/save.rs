use crate::story_graph::types::{StoryFlags, GraphExecutor, FlagValue};
use bevy::prelude::*;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

pub struct SavePlugin;

impl Plugin for SavePlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<SaveEvent>()
            .add_message::<LoadEvent>()
            .add_systems(Update, (handle_save_event, handle_load_event));
    }
}

#[derive(Message, Debug, Clone)]
pub struct SaveEvent(pub String); // Slot name or path

#[derive(Message, Debug, Clone)]
pub struct LoadEvent(pub String);

#[derive(Serialize, Deserialize, Debug)]
pub struct GameSaveData {
    pub format_version: String,
    pub timestamp: f64,
    pub story_flags: HashMap<String, FlagValue>,
    pub inventory: crate::game::Inventory,
    pub quest_log: crate::game::QuestLog,
    pub executor: crate::story_graph::types::GraphExecutor,
    pub player_position: Vec3,
}

fn handle_save_event(
    mut events: MessageReader<SaveEvent>,
    flags: Res<StoryFlags>,
    executor: Res<GraphExecutor>,
    inventory: Res<crate::game::Inventory>,
    quest_log: Res<crate::game::QuestLog>,
    player_query: Query<&Transform, With<crate::editor::state::LogicalEntity>>,
) {
    for event in events.read() {
        let player_pos = player_query.iter().next().map(|t| t.translation).unwrap_or(Vec3::ZERO);
        
        let save_data = GameSaveData {
            format_version: "1.1".to_string(),
            timestamp: 0.0, // TODO: Use real timestamp
            story_flags: flags.0.clone(),
            inventory: (*inventory).clone(),
            quest_log: (*quest_log).clone(),
            executor: (*executor).clone(),
            player_position: player_pos,
        };

        match serde_json::to_string_pretty(&save_data) {
            Ok(json) => {
                let path = format!("{}.json", event.0);
                if let Err(e) = fs::write(&path, json) {
                    error!("Failed to write save file {}: {}", path, e);
                } else {
                    info!("Robust game state saved to {}", path);
                }
            }
            Err(e) => error!("Failed to serialize save data: {}", e),
        }
    }
}

fn handle_load_event(
    mut events: MessageReader<LoadEvent>,
    mut flags: ResMut<StoryFlags>,
    mut executor: ResMut<GraphExecutor>,
    mut inventory: ResMut<crate::game::Inventory>,
    mut quest_log: ResMut<crate::game::QuestLog>,
    mut player_query: Query<&mut Transform, With<crate::editor::state::LogicalEntity>>,
) {
    for event in events.read() {
        let path = format!("{}.json", event.0);
        match fs::read_to_string(&path) {
            Ok(json) => {
                match serde_json::from_str::<GameSaveData>(&json) {
                    Ok(data) => {
                        *flags = StoryFlags(data.story_flags);
                        *executor = data.executor;
                        *inventory = data.inventory;
                        *quest_log = data.quest_log;
                        
                        // Force a state update for visibility in editor
                        if executor.active_graph_id.is_some() && executor.status == crate::story_graph::types::ExecutionStatus::Idle {
                           executor.status = crate::story_graph::types::ExecutionStatus::Paused;
                        }
                        
                        if let Some(mut transform) = player_query.iter_mut().next() {
                            transform.translation = data.player_position;
                        }
                        
                        info!("Robust game state loaded from {}", path);
                    }
                    Err(e) => error!("Failed to deserialize save file {}: {}", path, e),
                }
            }
            Err(e) => error!("Failed to read save file {}: {}", path, e),
        }
    }
}
