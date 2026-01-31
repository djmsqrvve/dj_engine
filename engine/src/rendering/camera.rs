//! Camera setup for DJ Engine rendering.
//!
//! Provides camera configuration for pixel-perfect rendering.

use bevy::prelude::*;

/// Marker component for the main game camera.
#[derive(Component)]
pub struct MainCamera;

/// The target resolution for the game canvas.
pub const GAME_WIDTH: f32 = 320.0;
pub const GAME_HEIGHT: f32 = 240.0;

/// Resource to track the available rendering area (from UI).
#[derive(Resource, Default)]
pub struct ViewportRect(pub Option<Rect>);

/// Sets up the main camera for 2D rendering.
pub fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Camera {
            order: -1,
            ..default()
        },
        Projection::from(OrthographicProjection {
            scale: 0.25, // Zoom in (4x)
            ..OrthographicProjection::default_2d()
        }),
        MainCamera,
        Transform::from_xyz(0.0, 0.0, 1000.0),
    ));
}

/// System to update the camera viewport based on the UI layout.
pub fn update_camera_viewport(
    viewport_rect: Res<ViewportRect>,
    mut camera_query: Query<&mut Camera, With<MainCamera>>,
    windows: Query<&Window, With<bevy::window::PrimaryWindow>>,
) {
    let Some(window) = windows.iter().next() else {
        return;
    };
    let Some(rect) = viewport_rect.0 else { return };
    let mut camera = match camera_query.iter_mut().next() {
        Some(c) => c,
        None => return,
    };

    // Bevy Viewport expects physical pixels or logical scaled?
    // Usually logical pixels if we use with_scale_factor_override(1.0) in main.rs
    // Egui rect is in logical pixels.

    let _physical_width = window.width();
    let _physical_height = window.height();

    // Ensure the rect is within window bounds to avoid wgpu panics
    let win_w = window.width();
    let win_h = window.height();

    let min_x = rect.min.x.clamp(0.0, win_w);
    let min_y = rect.min.y.clamp(0.0, win_h);
    let max_x = rect.max.x.clamp(0.0, win_w);
    let max_y = rect.max.y.clamp(0.0, win_h);

    let width = (max_x - min_x).max(1.0);
    let height = (max_y - min_y).max(1.0);

    camera.viewport = Some(bevy::camera::Viewport {
        physical_position: UVec2::new(min_x as u32, min_y as u32),
        physical_size: UVec2::new(width as u32, height as u32),
        depth: 0.0..1.0,
    });
}
