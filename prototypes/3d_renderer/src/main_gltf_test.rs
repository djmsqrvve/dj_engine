// Simple GLTF test that works with Bevy 0.14

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "GLTF Test (Press SPACE)".into(),
                resolution: (1920.0, 1080.0).into(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup_scene)
        .add_systems(Update, keyboard_input)
        .run();
}

fn setup_scene(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // Load Drow model
    let model_handle: Handle<Scene> = asset_server.load(
        "test_models/dota_models/models/heroes/drow/drow_base.gltf"
    );
    
    commands.spawn(model_handle);
    
    // Camera
    commands.spawn((Camera3d::default(), Transform::from_xyz(0.0, 5.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y)));
}

fn keyboard_input(
    keys: Res<ButtonInput<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::Space) {
        eprintln!("SPACE pressed - would capture frame if screenshot API available in 0.14");
    }
    if keys.just_pressed(KeyCode::Escape) {
        std::process::exit(0);
    }
}
