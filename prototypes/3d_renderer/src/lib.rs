// Main library module for tests

pub mod plugins;

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::prelude::*;

    #[test]
    fn test_camera_plugin_builds() {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, plugins::CameraPlugin));
        // Test passes if plugin builds without panic
    }

    #[test]
    fn test_lighting_plugin_builds() {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, plugins::LightingPlugin));
        // Test passes if plugin builds without panic
    }

    #[test]
    fn test_model_plugin_builds() {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, plugins::ModelPlugin, AssetPlugin::default()));
        // Test passes if plugin builds without panic
    }
}
