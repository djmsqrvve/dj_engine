use bevy::prelude::*;
use bevy_2d_renderer::{resources::*, state::AppState, systems::*, ui};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy 2D Rendering Sandbox".to_string(),
                resolution: (1280.0, 720.0).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(bevy_ecs_tilemap::TilemapPlugin)
        .insert_resource(ClearColor(Color::srgb(0.05, 0.05, 0.1)))
        .init_resource::<GameAssets>()
        .init_resource::<MousePosition>()
        .init_resource::<CameraSettings>()
        .insert_resource(DebugConsole::new(10))
        .init_state::<AppState>()
        .add_systems(Startup, setup_camera)
        .add_systems(Startup, setup_tilemap)
        .add_systems(Startup, setup_parallax_background)
        .add_systems(Startup, setup_player)
        .add_systems(Startup, setup_lighting)
        .add_systems(Startup, ui::setup_hud)
        .add_systems(Startup, setup_debug_console)
        .add_systems(Update, handle_camera_follow)
        .add_systems(Update, handle_camera_zoom)
        .add_systems(Update, update_mouse_position)
        .add_systems(Update, update_lighting_position)
        .add_systems(Update, animate_player)
        .add_systems(Update, update_parallax_layers)
        .add_systems(Update, update_debug_console.after(update_mouse_position))
        .run();
}
