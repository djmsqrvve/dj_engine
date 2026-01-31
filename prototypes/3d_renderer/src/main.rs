// Bevy 0.18 - GPU Test Scene with visible objects
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "GPU Test - Press ESC to exit".into(),
                resolution: (1920, 1080).into(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup_scene)
        .add_systems(Update, keyboard_input)
        .add_systems(Update, rotate_camera)
        .run();
}

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Ground plane - pale rose
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(20.0, 20.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.95, 0.85, 0.85),
            perceptual_roughness: 0.8,
            ..default()
        })),
    ));

    // Central cube - bright red (impossible to miss)
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(2.0, 2.0, 2.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(1.0, 0.2, 0.2),
            emissive: LinearRgba::rgb(0.2, 0.0, 0.0),
            ..default()
        })),
        Transform::from_xyz(0.0, 1.0, 0.0),
    ));

    // Surrounding spheres with different PBR materials
    let positions = [
        (-3.0, 1.0, -3.0),
        (3.0, 1.0, -3.0),
        (-3.0, 1.0, 3.0),
        (3.0, 1.0, 3.0),
    ];
    let colors = [
        Color::srgb(0.2, 0.8, 0.2), // Green
        Color::srgb(0.2, 0.2, 1.0), // Blue
        Color::srgb(1.0, 1.0, 0.2), // Yellow
        Color::srgb(0.8, 0.2, 0.8), // Purple
    ];

    for (i, (pos, color)) in positions.iter().zip(colors.iter()).enumerate() {
        commands.spawn((
            Mesh3d(meshes.add(Sphere::new(0.8 + i as f32 * 0.2))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: *color,
                metallic: (i as f32) * 0.25,
                perceptual_roughness: 0.1 + (i as f32) * 0.2,
                ..default()
            })),
            Transform::from_xyz(pos.0, pos.1, pos.2),
        ));
    }

    // Light
    commands.spawn((
        PointLight {
            intensity: 200000.0,
            color: Color::WHITE,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(5.0, 8.0, 5.0),
    ));

    // Sun
    commands.spawn((
        DirectionalLight {
            illuminance: 10000.0,
            color: Color::WHITE,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -0.5, 0.5, 0.0)),
    ));

    // Camera - positioned to see everything
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(8.0, 6.0, 12.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    info!("✅ Scene spawned: Red cube center, 4 colored spheres, ground plane");
    info!("Camera at (8, 6, 12) looking at origin");
}

fn rotate_camera(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Camera3d>>,
) {
    for mut transform in &mut query {
        // Slow orbit around center
        let t = time.elapsed_secs() * 0.2;
        let radius = 14.0;
        transform.translation.x = radius * t.cos();
        transform.translation.z = radius * t.sin();
        transform.look_at(Vec3::ZERO, Vec3::Y);
    }
}

fn keyboard_input(
    keys: Res<ButtonInput<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::Space) {
        info!("SPACE pressed");
    }
    if keys.just_pressed(KeyCode::Escape) {
        std::process::exit(0);
    }
}
