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

#[derive(Component)]
pub struct DebugConsoleUI;

#[derive(Component)]
pub struct Camera2DMarker;

#[derive(Component, Default)]
pub struct AnimationFrame {
    pub index: usize,
    pub layout: Handle<TextureAtlasLayout>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_animation_timer_creation() {
        let timer = AnimationTimer::new(0.5);
        assert_eq!(timer.timer.duration().as_secs_f32(), 0.5);
        assert!(matches!(timer.timer.mode(), TimerMode::Repeating));
    }

    #[test]
    fn test_point_light2d_creation() {
        let light = PointLight2D {
            intensity: 1.5,
            radius: 100.0,
            color: Color::srgb(0.0, 1.0, 0.5),
        };
        assert_eq!(light.intensity, 1.5);
        assert_eq!(light.radius, 100.0);
    }

    #[test]
    fn test_parallax_layer_creation() {
        let layer = ParallaxLayer { depth: 0.5 };
        assert_eq!(layer.depth, 0.5);
    }
}
