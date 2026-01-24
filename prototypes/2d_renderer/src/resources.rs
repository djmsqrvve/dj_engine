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
