use bevy::prelude::*;

#[derive(Resource)]
pub struct GameAssets {
    pub player_sprite: Handle<Image>,
    pub background_layer_1: Handle<Image>,
    pub background_layer_2: Handle<Image>,
    pub background_layer_3: Handle<Image>,
    pub tilemap_texture: Handle<Image>,
}

impl FromWorld for GameAssets {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.get_resource::<AssetServer>().unwrap();
        
        Self {
            player_sprite: asset_server.load("sprites/player.png"),
            background_layer_1: asset_server.load("backgrounds/layer1.png"),
            background_layer_2: asset_server.load("backgrounds/layer2.png"),
            background_layer_3: asset_server.load("backgrounds/layer3.png"),
            tilemap_texture: asset_server.load("tiles/tileset.png"),
        }
    }
}

#[derive(Resource, Default)]
pub struct MousePosition {
    pub world_position: Vec2,
}

#[derive(Resource)]
pub struct CameraSettings {
    pub follow_speed: f32,
    pub zoom_speed: f32,
    pub current_zoom: f32,
}

impl Default for CameraSettings {
    fn default() -> Self {
        Self {
            follow_speed: 5.0,
            zoom_speed: 0.1,
            current_zoom: 1.0,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

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
    fn test_mouse_position_custom() {
        let mouse_pos = MousePosition {
            world_position: Vec2::new(100.0, 200.0),
        };
        assert_eq!(mouse_pos.world_position.x, 100.0);
        assert_eq!(mouse_pos.world_position.y, 200.0);
    }

    #[test]
    fn test_camera_settings_custom() {
        let settings = CameraSettings {
            follow_speed: 10.0,
            zoom_speed: 0.2,
            current_zoom: 2.0,
        };
        assert_eq!(settings.follow_speed, 10.0);
        assert_eq!(settings.zoom_speed, 0.2);
        assert_eq!(settings.current_zoom, 2.0);
    }
}
