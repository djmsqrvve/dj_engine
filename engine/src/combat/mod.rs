use bevy::prelude::*;
use crate::data::components::{EntityMetadata, CombatStatsComponent};
use crate::story_graph::types::{GraphExecutor, StoryFlags};
use crate::editor::state::EditorUiState;

pub struct DJCombatPlugin;

impl Plugin for DJCombatPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ActiveCombat>()
            .add_message::<DamageEvent>()
            .add_message::<CombatActionEvent>()
            .add_systems(Update, (
                process_damage_system,
                check_death_system,
                update_combat_queue,
                process_battle_triggers,
                update_editor_node_trace,
                process_combat_turns,
            ).chain());
    }
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Reflect)]
pub enum Team {
    Player,
    Enemy,
}

#[derive(Resource, Default, Debug, Reflect)]
#[reflect(Resource)]
pub struct ActiveCombat {
    pub is_active: bool,
    pub turn_order: Vec<Entity>,
    pub current_turn_index: usize,
    pub round: u32,
}

#[derive(Message, Debug, Clone)]
pub struct CombatActionEvent {
    pub source: Entity,
    pub target: Entity,
    pub action_id: String,
}

#[derive(Message, Debug, Clone)]
pub struct DamageEvent {
    pub target: Entity,
    pub amount: f32,
}

fn process_damage_system(
    mut events: MessageReader<DamageEvent>,
    mut query: Query<(&mut CombatStatsComponent, &Team)>,
) {
    for event in events.read() {
        if let Ok((mut stats, _team)) = query.get_mut(event.target) {
            stats.hp = (stats.hp as f32 - event.amount).max(0.0) as i32;
            info!("Entity {:?} took {} damage, health: {}", event.target, event.amount, stats.hp);
        }
    }
}

fn check_death_system(
    query: Query<(Entity, &CombatStatsComponent, &Team)>,
    mut combat: ResMut<ActiveCombat>,
) {
    let mut players_alive = 0;
    let mut enemies_alive = 0;

    for (_entity, stats, team) in query.iter() {
        if stats.hp > 0 {
            match team {
                Team::Player => players_alive += 1,
                Team::Enemy => enemies_alive += 1,
            }
        }
    }

    if combat.is_active && (players_alive == 0 || enemies_alive == 0) {
        combat.is_active = false;
        info!("Combat ended. Players alive: {}, Enemies alive: {}", players_alive, enemies_alive);
    }
}

fn update_combat_queue(
    mut combat: ResMut<ActiveCombat>,
    query: Query<(Entity, &Team), With<EntityMetadata>>,
) {
    if !combat.is_active { return; }
    
    if combat.turn_order.is_empty() {
        let participants: Vec<_> = query.iter().collect();
        // Sort by some initiative (TODO)
        combat.turn_order = participants.iter().map(|(e, _)| *e).collect();
        combat.current_turn_index = 0;
    }
}

fn process_battle_triggers(
    _story_executor: ResMut<GraphExecutor>,
    _combat: ResMut<ActiveCombat>,
    _query: Query<(Entity, &Team), With<EntityMetadata>>,
) {
    // If story says we need battle, start it
    // (This is a simplified bridge)
}

fn update_editor_node_trace(
    story_executor: Res<GraphExecutor>,
    mut ui_state: ResMut<EditorUiState>,
) {
    if let Some(active) = story_executor.current_node {
        let active_id = format!("node_{}", active); // Use node_N format for trace matching
        if ui_state.node_trace.last() != Some(&active_id) {
            ui_state.node_trace.push(active_id);
            if ui_state.node_trace.len() > 10 {
                ui_state.node_trace.remove(0);
            }
        }
    }
}

fn process_combat_turns(
    combat: ResMut<ActiveCombat>,
) {
    if !combat.is_active { return; }
    // Combat logic...
}
