// Integration tests

use bevy::prelude::*;

#[test]
fn test_minimal_app_startup() {
    // Test that we can create and run a basic Bevy app
    let mut app = App::new();
    
    app.add_plugins(MinimalPlugins);
    
    // Add a simple system that spawns something
    app.add_systems(Startup, |mut commands: Commands| {
        commands.spawn(Transform::default());
    });
    
    // Run the app - should not panic
    app.update();
    
    // Verify we have entities
    assert!(app.world().entities().len() > 0);
}

#[test]
fn test_camera_creation() {
    // Test that we can create a camera
    let mut app = App::new();
    
    app.add_plugins(MinimalPlugins);
    
    // Add a system to spawn camera
    app.add_systems(Startup, |mut commands: Commands| {
        commands.spawn((
            Camera3d::default(),
            Transform::from_xyz(0.0, 5.0, 10.0),
        ));
    });
    
    app.update();
    
    // Verify we have at least the camera entity
    assert!(app.world().entities().len() > 0);
}
