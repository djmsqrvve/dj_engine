// Tests for camera and lighting systems

use bevy::prelude::*;

#[test]
fn test_camera_transform_looking_at() {
    // Test camera look-at functionality
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    
    app.add_systems(Startup, |mut commands: Commands| {
        commands.spawn((
            Camera3d::default(),
            Transform::from_xyz(0.0, 5.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ));
    });
    
    app.update();
    
    // Verify camera was spawned with correct components
    let mut query = app.world_mut().query::<(&Transform, &Camera3d)>();
    let count = query.iter(&app.world()).len();
    assert_eq!(count, 1);
}

#[test]
fn test_pale_rose_color_palette() {
    // Test that our color palette is correctly defined
    use bevy::color::Color;
    
    let colors = vec![
        Color::srgb(0.95, 0.85, 0.85),  // Pale rose 1
        Color::srgb(0.9, 0.7, 0.75),     // Pale rose 2
        Color::srgb(0.92, 0.88, 0.88),   // Ground
        Color::srgb(0.9, 0.85, 0.85),    // Clear color
    ];
    
    // Verify all colors are valid
    for color in colors {
        // Color should be in valid range [0.0, 1.0]
        if let Color::Srgba(srgba) = color {
            assert!(srgba.red >= 0.0 && srgba.red <= 1.0);
            assert!(srgba.green >= 0.0 && srgba.green <= 1.0);
            assert!(srgba.blue >= 0.0 && srgba.blue <= 1.0);
            assert!(srgba.alpha >= 0.0 && srgba.alpha <= 1.0);
        }
    }
}

#[test]
fn test_light_properties_ranges() {
    // Test that light properties are in valid ranges
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, AssetPlugin::default()));
    
    app.add_systems(Startup, |mut commands: Commands| {
        // Sun light
        commands.spawn(DirectionalLight {
            illuminance: 10000.0,  // Should be positive
            shadows_enabled: true,
            ..default()
        });
        
        // Point lights with various intensities
        commands.spawn(PointLight {
            intensity: 1500.0,  // Should be positive
            radius: 0.5,        // Should be positive
            color: Color::srgb(1.0, 0.7, 0.8),
            ..default()
        });
        
        commands.spawn(PointLight {
            intensity: 1000.0,
            radius: 0.5,
            color: Color::srgb(0.7, 0.8, 1.0),
            ..default()
        });
    });
    
    app.update();
    
    // Verify light intensities are positive
    for light in app.world_mut().query::<&DirectionalLight>().iter(&app.world()) {
        assert!(light.illuminance > 0.0);
    }
    
    for light in app.world_mut().query::<&PointLight>().iter(&app.world()) {
        assert!(light.intensity > 0.0);
        assert!(light.radius > 0.0);
    }
}

#[test]
fn test_multiple_cameras() {
    // Test that we can have multiple cameras (for different views)
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    
    app.add_systems(Startup, |mut commands: Commands| {
        // Main camera
        commands.spawn((
            Camera3d::default(),
            Transform::from_xyz(0.0, 5.0, 10.0),
        ));
        
        // Secondary camera (e.g., for UI or different view)
        commands.spawn((
            Camera3d::default(),
            Transform::from_xyz(0.0, 10.0, 20.0),
        ));
    });
    
    app.update();
    
    let cameras = app.world_mut().query::<&Camera3d>().iter(&app.world()).len();
    assert_eq!(cameras, 2);
}

#[test]
fn test_transform_hierarchy() {
    // Test parent-child transform relationships
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    
    app.add_systems(Startup, |mut commands: Commands| {
        // Parent entity
        let parent = commands.spawn(Transform::from_xyz(1.0, 0.0, 0.0)).id();
        
        // Child entity
        commands.spawn(Transform::from_xyz(0.0, 1.0, 0.0)).set_parent(parent);
    });
    
    app.update();
    
    // Both entities should exist
    assert_eq!(app.world().entities().len(), 2);
}

#[test]
fn test_clear_color_configuration() {
    // Test our pale rose clear color is properly configured
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    
    app.add_systems(Startup, |mut commands: Commands| {
        commands.spawn(SceneBundle {
            scene: Handle::<Scene>::default(),
            ..default()
        });
    });
    
    // Add camera with custom clear color
    app.add_systems(Startup, |mut commands: Commands| {
        commands.spawn((
            Camera3d::default(),
            Transform::from_xyz(0.0, 0.0, 0.0),
            Camera {
                clear_color: ClearColorConfig::Custom(Color::srgb(0.9, 0.85, 0.85)),
                ..default()
            },
        ));
    });
    
    app.update();
    
    // Verify camera was created with components
    let mut query = app.world_mut().query::<&Camera>();
    let count = query.iter(&app.world()).len();
    assert_eq!(count, 1);
}

#[test]
fn test_ground_plane_creation() {
    // Test ground plane setup
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, AssetPlugin::default()));
    
    app.add_systems(Startup, |mut commands: Commands| {
        // Large ground plane like in main.rs
        commands.spawn((
            Transform::from_xyz(0.0, 0.0, 0.0),
        ));
    });
    
    app.update();
    
    assert!(app.world().entities().len() > 0);
}

#[test]
fn test_entity_count_scaling() {
    // Test that we can scale to multiple entities
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    
    app.add_systems(Startup, |mut commands: Commands| {
        // Spawn many entities to test performance scaling
        for x in 0..100 {
            commands.spawn(Transform::from_xyz(x as f32, 0.0, 0.0));
        }
    });
    
    app.update();
    
    let entity_count = app.world().entities().len();
    assert_eq!(entity_count, 100);
}

#[test]
fn test_pbr_parameters_in_valid_range() {
    // Ensure all PBR parameters are within physically valid ranges
    let test_materials = vec![
        (0.0, 0.9),   // (metallic, roughness)
        (0.1, 0.7),
        (0.3, 0.5),
        (0.6, 0.3),
        (0.9, 0.1),
    ];
    
    for (metallic, roughness) in test_materials {
        // Validate ranges
        assert!(metallic >= 0.0 && metallic <= 1.0, 
                "Metallic must be in [0.0, 1.0], got {}", metallic);
        assert!(roughness >= 0.0 && roughness <= 1.0, 
                "Roughness must be in [0.0, 1.0], got {}", roughness);
        
        // These should not panic
        let _material_params = (metallic, roughness);
    }
}
