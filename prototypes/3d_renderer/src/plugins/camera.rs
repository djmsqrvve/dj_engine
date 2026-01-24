use bevy::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, orbit_camera);
    }
}

#[derive(Component)]
pub struct OrbitCamera {
    pub center: Vec3,
    pub radius: f32,
    pub yaw: f32,
    pub pitch: f32,
}

impl Default for OrbitCamera {
    fn default() -> Self {
        Self {
            center: Vec3::ZERO,
            radius: 10.0,
            yaw: 0.0,
            pitch: 0.3,
        }
    }
}

fn orbit_camera(
    mut camera_query: Query<(&mut Transform, &OrbitCamera), With<Camera3d>>,
    _time: Res<Time>,
) {
    for (mut transform, orbit_cam) in camera_query.iter_mut() {
        let x = orbit_cam.radius * orbit_cam.yaw.cos() * orbit_cam.pitch.cos();
        let y = orbit_cam.radius * orbit_cam.pitch.sin();
        let z = orbit_cam.radius * orbit_cam.yaw.sin() * orbit_cam.pitch.cos();

        *transform = Transform::from_xyz(x, y, z).looking_at(orbit_cam.center, Vec3::Y);
    }
}
