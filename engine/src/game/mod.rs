use bevy::prelude::*;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

pub struct DJGamePlugin;

impl Plugin for DJGamePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Inventory>()
            .init_resource::<QuestLog>()
            .register_type::<Inventory>()
            .register_type::<QuestLog>();
    }
}

#[derive(Resource, Default, Debug, Clone, Reflect, Serialize, Deserialize)]
#[reflect(Resource)]
pub struct Inventory {
    pub items: HashMap<String, u32>, // item_id -> quantity
}

impl Inventory {
    pub fn add_item(&mut self, item_id: &str, quantity: u32) {
        *self.items.entry(item_id.to_string()).or_insert(0) += quantity;
        info!("Inventory: Added {}x {}", quantity, item_id);
    }

    pub fn remove_item(&mut self, item_id: &str, quantity: u32) -> bool {
        if let Some(q) = self.items.get_mut(item_id) {
            if *q >= quantity {
                *q -= quantity;
                if *q == 0 {
                    self.items.remove(item_id);
                }
                return true;
            }
        }
        false
    }

    pub fn has_item(&self, item_id: &str, quantity: u32) -> bool {
        self.items.get(item_id).map_or(false, |q| *q >= quantity)
    }
}

#[derive(Resource, Default, Debug, Clone, Reflect, Serialize, Deserialize)]
#[reflect(Resource)]
pub struct QuestLog {
    pub active_quests: HashMap<String, QuestState>,
    pub completed_quests: Vec<String>,
}

#[derive(Debug, Clone, Reflect, Serialize, Deserialize, PartialEq)]
pub enum QuestState {
    NotStarted,
    Active(String), // Current objective ID
    ReadyToTurnIn,
    Completed,
}

impl QuestLog {
    pub fn set_state(&mut self, quest_id: &str, state: QuestState) {
        if state == QuestState::Completed {
            self.active_quests.remove(quest_id);
            if !self.completed_quests.contains(&quest_id.to_string()) {
                self.completed_quests.push(quest_id.to_string());
            }
        } else {
            self.active_quests.insert(quest_id.to_string(), state);
        }
        info!("QuestLog: {} -> {:?}", quest_id, self.active_quests.get(quest_id));
    }
}
