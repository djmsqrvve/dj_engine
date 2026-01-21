use bevy::prelude::*;
use crate::state::GameState;

mod systems;
mod ui;

pub struct BattlePlugin;

impl Plugin for BattlePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<BattleResultEvent>()
            .add_systems(OnEnter(GameState::Battle), ui::setup_battle_ui)
            .add_systems(
                Update,
                (
                    ui::battle_ui_interaction,
                    systems::handle_battle_result,
                ).run_if(in_state(GameState::Battle)),
            )
            .add_systems(OnExit(GameState::Battle), ui::cleanup_battle_ui);
    }
}

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[allow(dead_code)]
pub enum BattleState {
    #[default]
    Idle, // Waiting for battle to start
    InBattle,
    Victory,
    Defeat,
}

#[derive(Event)]
pub enum BattleResultEvent {
    Win,
    Lose,
}
