use bevy::prelude::*;

pub struct LightingPlugin;

impl Plugin for LightingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_lighting)
            .add_systems(Update, animate_point_lights);
    }
}

fn setup_lighting(mut commands: Commands) {
    commands.spawn((
        DirectionalLight {
            color: Color::srgb(1.0, 0.9, 0.8),
            illuminance: 10000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    commands.spawn((
        PointLight {
            color: Color::srgb(1.0, 0.7, 0.8),
            intensity: 1500.0,
            radius: 0.5,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(3.0, 2.0, 0.0),
    ));

    commands.spawn((
        PointLight {
            color: Color::srgb(0.7, 0.8, 1.0),
            intensity: 1000.0,
            radius: 0.5,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(-3.0, 2.0, 0.0),
    ));
}

fn animate_point_lights(
    mut point_lights: Query<&mut Transform, With<PointLight>>,
    time: Res<Time>,
) {
    let t = time.elapsed_secs();

    for (i, mut transform) in point_lights.iter_mut().enumerate() {
        let offset = i as f32 * std::f32::consts::PI;
        let radius = 3.0;
        let speed = 0.5;

        transform.translation.x = (t * speed + offset).cos() * radius;
        transform.translation.z = (t * speed + offset).sin() * radius;
        transform.translation.y = 2.0 + (t * speed * 2.0 + offset).sin() * 0.5;
    }
}
