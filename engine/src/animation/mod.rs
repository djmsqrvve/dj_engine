//! Animation system for DJ Engine.
//!
//! Provides procedural breathing, blinking, and idle motion animations.

use bevy::prelude::*;

pub mod components;
pub mod events;
pub mod systems;

pub use components::{BlinkingAnimation, BreathingAnimation, IdleMotion};
pub use events::AnimationCommand;

use std::sync::{Arc, Mutex};

/// Thread-safe queue for animation commands from Lua.
#[derive(Resource, Clone, Default)]
pub struct SharedAnimationCommands(pub Arc<Mutex<Vec<AnimationCommand>>>);

/// Animation plugin that registers all animation systems.
pub struct DJAnimationPlugin;

impl Plugin for DJAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<AnimationCommand>()
            .init_resource::<SharedAnimationCommands>()
            .add_systems(
                Update,
                (
                    systems::breathing_system,
                    systems::blinking_system,
                    systems::expression_system,
                    systems::idle_motion_system,
                    systems::flush_animation_commands,
                    systems::handle_animation_commands,
                ),
            );
    }
}
