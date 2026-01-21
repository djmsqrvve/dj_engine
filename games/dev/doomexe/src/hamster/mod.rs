//! Hamster character module for doomexe.
//!
//! Provides the hamster narrator entity with component-driven animations,
//! expression switching, and corruption visual effects.
//!
//! ## Controls (Debug)
//! - `1`, `2`, `3`: Set expression (Neutral, Happy, Angry)
//! - `A`: Cycle expression
//! - `U`: Increase corruption
//! - `D`: Decrease corruption

mod assembly;
pub mod components;
mod systems;
mod tests;

pub use assembly::spawn_character;

use bevy::prelude::*;
// use crate::state::GameState; // Unused here

/// Plugin for the hamster character system.
pub struct HamsterPlugin;

impl Plugin for HamsterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_hamster).add_systems(
            Update,
            (
                systems::breathing_system,
                systems::blinking_system,
                systems::idle_motion_system,
                systems::expression_system,
                systems::corruption_system,
                systems::debug_input_system,
                systems::toggle_visibility_system,
            ),
        );
    }
}

/// Startup system to spawn the hamster character.
fn setup_hamster(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    spawn_character(&mut commands, &mut images);
}
