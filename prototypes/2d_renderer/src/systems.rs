use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use crate::components::*;
use crate::resources::*;

pub fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 1000.0),
            ..default()
        },
        MainCamera,
    ));
}

pub fn handle_camera_follow(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<MainCamera>, Without<Player>)>,
    time: Res<Time>,
    camera_settings: Res<CameraSettings>,
) {
    let player_transform = player_query.single();
    let mut camera_transform = camera_query.single_mut();
    
    let follow_speed = camera_settings.follow_speed;
    let target_position = player_transform.translation.truncate();
    
    camera_transform.translation = camera_transform.translation.lerp(
        target_position.extend(camera_transform.translation.z),
        follow_speed * time.delta_seconds(),
    );
}

pub fn handle_camera_zoom(
    mut camera_query: Query<&mut OrthographicProjection, With<MainCamera>>,
    mut camera_settings: ResMut<CameraSettings>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    let mut projection = camera_query.single_mut();
    
    if keyboard.pressed(KeyCode::Equal) {
        camera_settings.current_zoom -= camera_settings.zoom_speed;
    }
    if keyboard.pressed(KeyCode::Minus) {
        camera_settings.current_zoom += camera_settings.zoom_speed;
    }
    
    camera_settings.current_zoom = camera_settings.current_zoom.clamp(0.5, 2.0);
    projection.scale = camera_settings.current_zoom;
}

pub fn update_mouse_position(
    windows: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut mouse_position: ResMut<MousePosition>,
) {
    let window = windows.single();
    let (camera, camera_transform) = camera_query.single();
    
    if let Some(cursor_position) = window.cursor_position() {
        if let Some(world_position) = camera.viewport_to_world(camera_transform, cursor_position) {
            mouse_position.world_position = world_position.origin.truncate();
        }
    }
}

pub fn setup_lighting(mut commands: Commands) {
    let spawn_point = Vec3::new(0.0, 0.0, 10.0);
    
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_translation(spawn_point),
            sprite: Sprite {
                color: Color::srgba(0.0, 1.0, 0.5, 0.3),
                custom_size: Some(Vec2::new(200.0, 200.0)),
                ..default()
            },
            ..default()
        },
        PointLight2D {
            intensity: 1.0,
            radius: 200.0,
            color: Color::srgb(0.0, 1.0, 0.5),
        },
    ));
}

pub fn update_lighting_position(
    mut light_query: Query<&mut Transform, With<PointLight2D>>,
    mouse_position: Res<MousePosition>,
) {
    let mut light_transform = light_query.single_mut();
    light_transform.translation.x = mouse_position.world_position.x;
    light_transform.translation.y = mouse_position.world_position.y;
}

pub fn setup_player(mut commands: Commands, assets: Res<GameAssets>, mut atlases: ResMut<Assets<TextureAtlasLayout>>) {
    let texture_atlas_layout = TextureAtlasLayout::from_grid(
        Vec2::new(32.0, 32.0).as_uvec2(),
        4,
        1,
        None,
        None,
    );
    let texture_atlas_layout_handle = atlases.add(texture_atlas_layout);
    
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(0.0, 0.0, 1.0),
            texture: assets.player_sprite.clone(),
            ..default()
        },
        TextureAtlas {
            layout: texture_atlas_layout_handle,
            index: 0,
        },
        Player,
        AnimationTimer::new(0.1),
    ));
}

pub fn animate_player(
    time: Res<Time>,
    mut query: Query<(&mut AnimationTimer, &mut TextureAtlas), With<Player>>,
) {
    for (mut timer, mut atlas) in query.iter_mut() {
        timer.timer.tick(time.delta());
        
        if timer.timer.just_finished() {
            atlas.index = (atlas.index + 1) % 4;
        }
    }
}

pub fn setup_parallax_background(mut commands: Commands, assets: Res<GameAssets>) {
    let layers = vec![
        (assets.background_layer_3.clone(), 0.1, -2.0),
        (assets.background_layer_2.clone(), 0.5, -1.0),
        (assets.background_layer_1.clone(), 0.8, 0.0),
    ];
    
    for (texture, depth, z_pos) in layers {
        commands.spawn((
            SpriteBundle {
                texture,
                transform: Transform::from_xyz(0.0, 0.0, z_pos),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(1280.0, 720.0)),
                    ..default()
                },
                ..default()
            },
            ParallaxLayer { depth },
        ));
    }
}

pub fn update_parallax_layers(
    camera_query: Query<&Transform, With<MainCamera>>,
    mut parallax_query: Query<(&mut Transform, &ParallaxLayer), Without<MainCamera>>,
) {
    let camera_transform = camera_query.single();
    let camera_x = camera_transform.translation.x;
    
    for (mut transform, parallax) in parallax_query.iter_mut() {
        transform.translation.x = -(camera_x * parallax.depth);
    }
}

pub fn setup_tilemap(mut commands: Commands, assets: Res<GameAssets>) {
    let map_size = TilemapSize { x: 32, y: 24 };
    let tile_size = TilemapTileSize { x: 32.0, y: 32.0 };
    let grid_size = tile_size.into();
    let map_type = TilemapType::default();
    
    let mut tile_storage = TileStorage::empty(map_size);
    
    for x in 0..map_size.x {
        for y in 0..map_size.y {
            let tile_pos = TilePos { x, y };
            let tile_entity = commands
                .spawn(TileBundle {
                    position: tile_pos,
                    ..default()
                })
                .id();
            tile_storage.set(&tile_pos, tile_entity);
        }
    }
    
    commands.spawn((
        TilemapBundle {
            size: map_size,
            storage: tile_storage,
            texture: TilemapTexture::Single(assets.tilemap_texture.clone()),
            tile_size,
            grid_size,
            map_type,
            transform: Transform::from_xyz(-512.0, -384.0, -10.0),
            ..default()
        },
        TilemapLayer,
    ));
}
