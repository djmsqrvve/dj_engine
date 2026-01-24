//! Rendering system for DJ Engine.
//!
//! Provides offscreen rendering, palette swapping, and CRT post-processing.

use bevy::prelude::*;

pub mod camera;

pub use camera::{MainCamera, ViewportRect, GAME_HEIGHT, GAME_WIDTH};

/// Rendering plugin that sets up the visual pipeline.
pub struct RenderingPlugin;

impl Plugin for RenderingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ViewportRect>()
            .add_systems(Startup, camera::setup_camera)
            .add_systems(Update, camera::update_camera_viewport);

        // TODO: Setup offscreen render target (320×240)
        // TODO: Register CRT post-processing pass
    }
}
