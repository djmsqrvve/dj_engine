use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct PointLight2D {
    pub intensity: f32,
    pub radius: f32,
    pub color: Color,
}

#[derive(Component)]
pub struct ParallaxLayer {
    pub depth: f32,
}

#[derive(Component)]
pub struct AnimationTimer {
    pub timer: Timer,
}

impl AnimationTimer {
    pub fn new(duration: f32) -> Self {
        Self {
            timer: Timer::from_seconds(duration, TimerMode::Repeating),
        }
    }
}

#[derive(Component)]
pub struct TilemapLayer;
