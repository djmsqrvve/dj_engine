mod diagnostic_plugin;

use bevy::prelude::*;
use plugins::{CameraPlugin, LightingPlugin, ModelPlugin};
use diagnostic_plugin::DiagnosticPlugin;

mod plugins;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Bevy 3D Rendering Sandbox".into(),
                    resolution: (1920.0, 1080.0).into(),
                    ..default()
                }),
                ..default()
            }),
            CameraPlugin,
            LightingPlugin,
            ModelPlugin,
            DiagnosticPlugin, // Use diagnostic instead of debug
        ))
        .add_systems(Startup, (
            setup_camera,
            setup_ground,
        ))
        .add_systems(Update, debug_status)
        .run();
}

fn setup_camera(mut commands: Commands) {
    info!("Spawning camera at position (0, 5, 10) looking at origin");
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 5.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        Camera {
            clear_color: ClearColorConfig::Custom(Color::srgb(0.9, 0.85, 0.85)),
            ..default()
        },
    ));
}

fn setup_ground(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    info!("Spawning ground plane at y=0 (50x50 units)");
    commands.spawn((
        meshes.add(Plane3d::new(Vec3::Y, Vec2::new(50.0, 50.0))),
        materials.add(StandardMaterial {
            base_color: Color::srgb(0.92, 0.88, 0.88),
            perceptual_roughness: 0.8,
            metallic: 0.0,
            ..default()
        }),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
}

fn debug_status(
    query: Query<&Transform, With<Camera3d>>,
    entity_query: Query<(), With<Handle<Mesh>>>,
) {
    for transform in query.iter() {
        info!("Camera position: {:?}", transform.translation);
        info!("Camera looking at: {:?}", transform.forward());
    }
    
    if entity_query.iter().len() > 0 {
        info!("Total entities with meshes: {}", entity_query.iter().len());
    }
}

