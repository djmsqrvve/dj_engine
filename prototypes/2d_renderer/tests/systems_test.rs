use bevy::prelude::*;
use bevy_2d_renderer::*;

#[test]
fn test_animation_timer_creation() {
    let timer = AnimationTimer::new(0.5);
    assert_eq!(timer.timer.duration().as_secs_f32(), 0.5);
    assert!(matches!(timer.timer.mode(), TimerMode::Repeating));
}

#[test]
    assert_eq!(console.max_messages, 5);
    assert_eq!(console.messages.len(), 0);
}

#[test]
    
    console.log("Message 1".to_string());
    console.log("Message 2".to_string());
    console.log("Message 3".to_string());
    
    assert_eq!(console.messages.len(), 3);
    assert_eq!(console.messages[0], "Message 1");
    assert_eq!(console.messages[1], "Message 2");
    assert_eq!(console.messages[2], "Message 3");
    
    // Adding a 4th message should remove the first
    console.log("Message 4".to_string());
    assert_eq!(console.messages.len(), 3);
    assert_eq!(console.messages[0], "Message 2");
    assert_eq!(console.messages[1], "Message 3");
    assert_eq!(console.messages[2], "Message 4");
}

#[test]
    console.log("Test message".to_string());
    assert_eq!(console.messages.len(), 1);
    
    console.clear();
    assert_eq!(console.messages.len(), 0);
}

#[test]
    console.log("Msg 1".to_string());
    console.log("Msg 2".to_string());
    
    let messages = console.get_messages();
    assert_eq!(messages.len(), 2);
    assert_eq!(messages[0], "Msg 1");
    assert_eq!(messages[1], "Msg 2");
}

#[test]
fn test_mouse_position_default() {
    let mouse_pos = MousePosition::default();
    assert_eq!(mouse_pos.world_position, Vec2::ZERO);
}

#[test]
fn test_camera_settings_default() {
    let settings = CameraSettings::default();
    assert_eq!(settings.follow_speed, 5.0);
    assert_eq!(settings.zoom_speed, 0.1);
    assert_eq!(settings.current_zoom, 1.0);
}

#[test]
fn test_app_state_transitions() {
    let loading = AppState::Loading;
    let playing = AppState::Playing;
    let paused = AppState::Paused;
    
    assert_eq!(loading, AppState::Loading);
    assert_eq!(playing, AppState::Playing);
    assert_eq!(paused, AppState::Paused);
    assert_ne!(loading, playing);
    assert_ne!(playing, paused);
}

#[test]
fn test_game_state_transitions() {
    let none = GameState::None;
    let moving = GameState::ProtagonistMoving;
    
    assert_eq!(none, GameState::None);
    assert_eq!(moving, GameState::ProtagonistMoving);
    assert_ne!(none, moving);
}

#[test]
fn test_point_light2d_creation() {
    let light = PointLight2D {
        intensity: 2.0,
        radius: 150.0,
        color: Color::srgb(0.0, 1.0, 0.5),
    };
    
    assert_eq!(light.intensity, 2.0);
    assert_eq!(light.radius, 150.0);
}

#[test]
fn test_parallax_layer_creation() {
    let layer = ParallaxLayer { depth: 0.7 };
    assert_eq!(layer.depth, 0.7);
}

// Integration test for Bevy App creation
#[test]
fn test_bevy_app_creation() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .init_resource::<MousePosition>()
        .init_resource::<CameraSettings>();
    // Note: Skipping AppState init to avoid complexity in minimal test
    // State requires more complete plugin setup
    
    app.update();
    
    assert!(app.world().contains_resource::<MousePosition>());
    assert!(app.world().contains_resource::<CameraSettings>());
}

// Test component spawning
#[test]
fn test_component_spawning() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    
    let entity = app.world_mut().spawn((
        Player,
        AnimationTimer::new(0.1),
        PointLight2D {
            intensity: 1.0,
            radius: 100.0,
            color: Color::WHITE,
        },
    )).id();
    
    assert!(app.world().entity(entity).contains::<Player>());
    assert!(app.world().entity(entity).contains::<AnimationTimer>());
    assert!(app.world().entity(entity).contains::<PointLight2D>());
}
