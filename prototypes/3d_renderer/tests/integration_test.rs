// Integration tests

use bevy::prelude::*;
use bevy_3d_renderer::plugins::*;

#[test]
fn test_full_app_initialization() {
    let mut app = App::new();
    app.add_plugins((
        MinimalPlugins,
        AssetPlugin::default(),
        CameraPlugin,
        LightingPlugin,
        ModelPlugin,
    ));
    
    app.update(); // Run one frame
    
    // Check that window resource exists
    assert!(app.world().contains_resource::<bevy::window::Window>());
}

#[test]
fn test_scene_setup() {
    let mut app = App::new();
    
    app.add_plugins((
        MinimalPlugins,
        AssetPlugin::default(),
    ));
    
    app.add_systems(Startup, |mut commands: Commands| {
        commands.spawn(Camera3d::default());
    });
    
    app.update();
    
    // Verify camera exists
    let cameras = app.world().query::<&Camera3d>().iter(&app.world()).len();
    assert!(cameras > 0);
}
