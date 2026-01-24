use bevy::prelude::*;
use plugins::{CameraPlugin, LightingPlugin, ModelPlugin};

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
        ))
        .add_systems(Startup, (setup_camera, setup_ground))
        .run();
}

fn setup_camera(mut commands: Commands) {
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
