use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "GLTF Capture - Press SPACE".into(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, load_gltf)
        .add_systems(Update, manual_capture)
        .run();
}

fn load_gltf(mut commands: Commands, asset_server: Res<AssetServer>) {
    error!("Loading Drow Ranger model...");
    commands.spawn(SceneRoot(
        asset_server.load("test_models/dota_models/models/heroes/drow/drow_base.gltf")
    ));
    
    commands.spawn((Camera3d::default(), Transform::from_xyz(0.0, 5.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y)));
    error!("Model loaded and camera positioned");
}

fn manual_capture(
    keys: Res<ButtonInput<KeyCode>>,
    mut screenshot_manager: ResMut<ScreenshotManager>,
    windows: Query<Entity, With<PrimaryWindow>>,
) {
    if keys.just_pressed(KeyCode::Space) {
        if let Ok(window) = windows.get_single() {
            let filename = format!("drow_{}.png", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs());
            match screenshot_manager.save_screenshot_to_disk(window, PathBuf::from("captures").join(&filename)) {
                Ok(()) => error!("✓ Captured {} (this should save in Bevy 0.14+ with features)", filename),
                Err(e) => error!("✗ Capture failed: {}", e),
            }
        }
    }
}
