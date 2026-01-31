use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use crate::components::*;
use crate::resources::*;

pub fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera::default(),
        Projection::from(OrthographicProjection {
            scale: 1.0,
            area: bevy::math::Rect::new(-640.0, -360.0, 640.0, 360.0),
            far: 1000.0,
            near: -1000.0,
            viewport_origin: Vec2::new(0.5, 0.5),
            scaling_mode: Default::default(),
        }),
        Transform::from_xyz(0.0, 0.0, 1000.0),
        MainCamera,
    ));
}


