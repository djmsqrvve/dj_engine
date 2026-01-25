// Main.rs with manual screenshot capture on SPACE

use bevy::prelude::*;
use bevy::render::view::screenshot::ScreenshotManager;
use std::path::PathBuf;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy 3D Renderer (Press SPACE to capture)".into(),
                resolution: (1920.0, 1080.0).into(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup_scene)
        .add_systems(Update, (
            print_info,
            spacebar_capture,
        ))
        .run();
}

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Drow Ranger Model (your GLTF)
    info!("Loading Drow Ranger model...");
    commands.spawn(SceneRoot(
        asset_server().load("test_models/dota_models/models/heroes/drow/drow_base.gltf")
    ));
    
    // Camera positioned to see the model
    commands.spawn((Camera3d::default(), Transform::from_xyz(0.0, 5.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y)));
    
    // Pale rose ground
    commands.spawn((
        meshes.add(Plane3d::new(Vec3::Y, Vec2::new(50.0, 50.0))),
        materials.add(StandardMaterial {
            base_color: Color::srgb(0.92, 0.88, 0.88),
            ..default()
        }),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
}

fn print_info(
    mesh_query: Query<&Handle<Mesh>>,
    time: Res<Time>,
) {
    if (time.elapsed_seconds() as i32) % 5 == 0 && time.elapsed_seconds().fract() < 0.1 {
        eprintln!("=== [${:.1}s] Entities with meshes: {}", 
                  time.elapsed_seconds(), 
                  mesh_query.iter().len());
    }
}

fn spacebar_capture(
    keys: Res<ButtonInput<KeyCode>>,
    mut screenshot_manager: ResMut<ScreenshotManager>,
    main_window: Query<Entity, With<PrimaryWindow>>,
    mut capture_count: Local<u32>,
) {
    if keys.just_pressed(KeyCode::Space) {
        *capture_count += 1;
        
        let filename = format!("drow_capture_{:03}.png", capture_count);
        let filepath = PathBuf::from("./captures").join(&filename);
        
        std::fs::create_dir_all("./captures").unwrap();
        
        if let Ok(window) = main_window.get_single() {
            match screenshot_manager.save_screenshot_to_disk(window, &filepath) {
                Ok(()) => {
                    eprintln!("╔═══════════════════════════════════════════════════════════╗");
                    eprintln!("║ ✓ CAPTURED: {} ", filename);
                    eprintln!("║   Location: {:?}", filepath);
                    eprintln!("╚═══════════════════════════════════════════════════════════╝");
                }
                Err(e) => {
                    eprintln!("✗ Capture failed: {}", e);
                }
            }
        }
    }
}

fn asset_server() -> AssetServer {
    // This is a workaround - in real code, use Res<AssetServer>
    unimplemented!("Use Res<AssetServer> in actual implementation") 
}
