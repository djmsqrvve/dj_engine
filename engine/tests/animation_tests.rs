use bevy::prelude::*;
use bevy::time::TimePlugin;
use dj_engine::prelude::*;
use dj_engine::animation::components::{BlinkingAnimation, BreathingAnimation, ExpressionController};

fn setup_app() -> App {
    let mut app = App::new();
    // Use MinimalPlugins but disable TimePlugin so we can control time manually
    app.add_plugins(MinimalPlugins.build().disable::<TimePlugin>());
    app.init_resource::<Time>();
    app.add_plugins(DJAnimationPlugin);
    app
}

#[test]
fn test_breathing_animation() {
    let mut app = setup_app();

    // Spawn entity with breathing
    let entity = app
        .world_mut()
        .spawn((
            Transform::default(),
            BreathingAnimation::new(0.1, 1.0), // 10% amplitude, 1Hz
        ))
        .id();

    // Initial state
    let transform = app.world().get::<Transform>(entity).unwrap();
    assert_eq!(transform.scale, Vec3::ONE);

    // Advance time (0.25s = 1/4 cycle = peak of sine wave)
    let mut time = app.world_mut().resource_mut::<Time>();
    time.advance_by(std::time::Duration::from_secs_f32(0.25));
    
    // Run systems
    app.update();

    let transform = app.world().get::<Transform>(entity).unwrap();
    // At peak (0.25s), sin(pi/2) = 1.
    // Scale Y should be 1.0 + 0.1 = 1.1
    // Scale X should be slightly less than 1.0 (squash)
    assert!((transform.scale.y - 1.1).abs() < 0.01, "Y scale should expand. Got {}", transform.scale.y);
    assert!(transform.scale.x < 1.0, "X scale should contract");
}

#[test]
fn test_blinking_animation() {
    let mut app = setup_app();

    // Spawn entity with blinking (eyes)
    let entity = app
        .world_mut()
        .spawn((
            Visibility::Inherited,
            BlinkingAnimation::new(0.1, 1.0, 1.0), // 0.1s blink, 1.0s interval
        ))
        .id();

    // Initial state: Eyes open (timer counting down)
    let vis = app.world().get::<Visibility>(entity).unwrap();
    assert_eq!(*vis, Visibility::Inherited);

    // Advance time to trigger blink (1.0s interval)
    let mut time = app.world_mut().resource_mut::<Time>();
    time.advance_by(std::time::Duration::from_secs_f32(1.1));

    app.update();

    // Should be blinking now (Hidden)
    let vis = app.world().get::<Visibility>(entity).unwrap();
    assert_eq!(*vis, Visibility::Hidden, "Eyes should be closed (Hidden)");

    // Advance time to end blink (0.1s duration)
    let mut time = app.world_mut().resource_mut::<Time>();
    time.advance_by(std::time::Duration::from_secs_f32(0.2));

    app.update();

    // Should be open again
    let vis = app.world().get::<Visibility>(entity).unwrap();
    assert_eq!(*vis, Visibility::Inherited, "Eyes should be open again");
}

#[test]
fn test_expression_controller() {
    let mut app = setup_app();
    app.add_plugins(AssetPlugin::default());
    app.init_asset::<TextureAtlasLayout>();

    // Spawn entity with expression controller
    let mut controller = ExpressionController::new();
    controller.expressions.insert("happy".to_string(), 5);
    
    // Create a mock texture atlas
    let layout = TextureAtlasLayout::from_grid(UVec2::new(16, 16), 4, 4, None, None);
    let layout_handle = app.world_mut().resource_mut::<Assets<TextureAtlasLayout>>().add(layout);

    let entity = app
        .world_mut()
        .spawn((
            Sprite {
                texture_atlas: Some(TextureAtlas {
                    layout: layout_handle,
                    index: 0,
                }),
                ..default()
            },
            controller,
        ))
        .id();

    app.update();

    // Change expression
    let mut controller = app.world_mut().get_mut::<ExpressionController>(entity).unwrap();
    controller.current_expression = "happy".to_string();

    app.update();

    // Verify index updated
    let sprite = app.world().get::<Sprite>(entity).unwrap();
    let atlas = sprite.texture_atlas.as_ref().unwrap();
    assert_eq!(atlas.index, 5, "Sprite index should update to 'happy' (5)");
}

#[test]
fn test_animation_stress_stability() {
    let mut app = setup_app();

    // Spawn 1000 entities
    for _ in 0..1000 {
        app.world_mut().spawn((
            Transform::default(),
            BreathingAnimation::hamster_default(),
            BlinkingAnimation::hamster_default(),
        ));
    }

    // Run for 10 frames
    for _ in 0..10 {
        let mut time = app.world_mut().resource_mut::<Time>();
        time.advance_by(std::time::Duration::from_secs_f32(0.016));
        app.update();
    }

    // Pass if no panic
    assert_eq!(app.world().entities().len(), 1000);
}
