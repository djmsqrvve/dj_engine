use bevy::prelude::*;
use bevy::gltf::GltfAssetLabel;

pub struct ModelPlugin;

impl Plugin for ModelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (load_gltf_models, spawn_pbr_test_objects))
            .add_systems(Update, check_gltf_loaded.run_if(resource_exists::<DrowAsset>));
    }
}

#[derive(Resource)]
struct DrowAsset {
    scene: Handle<Scene>,
}

fn load_gltf_models(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let drow_scene = asset_server.load(
        GltfAssetLabel::Scene(0).from_asset("test_models/dota_models/models/heroes/drow/drow_base.gltf")
    );
    
    commands.insert_resource(DrowAsset {
        scene: drow_scene,
    });
}

fn check_gltf_loaded(
    mut commands: Commands,
    mut done: Local<bool>,
    drow_asset: Res<DrowAsset>,
    asset_server: Res<AssetServer>,
) {
    if *done {
        return;
    }
    
    if asset_server.load_state(&drow_asset.scene) == bevy::asset::LoadState::Loaded {
        info!("Drow model loaded successfully! Spawning scene...");
        commands.spawn((
            SceneBundle {
                scene: drow_asset.scene.clone(),
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                ..default()
            },
        ));
        *done = true;
    }
}

fn spawn_pbr_test_objects(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let pale_rose_materials = vec![
        StandardMaterial {
            base_color: Color::srgb(0.95, 0.85, 0.85),
            metallic: 0.0,
            perceptual_roughness: 0.9,
            ..default()
        },
        StandardMaterial {
            base_color: Color::srgb(0.9, 0.7, 0.75),
            metallic: 0.1,
            perceptual_roughness: 0.7,
            ..default()
        },
        StandardMaterial {
            base_color: Color::srgb(0.85, 0.75, 0.8),
            metallic: 0.3,
            perceptual_roughness: 0.5,
            ..default()
        },
        StandardMaterial {
            base_color: Color::srgb(0.8, 0.65, 0.7),
            metallic: 0.6,
            perceptual_roughness: 0.3,
            ..default()
        },
        StandardMaterial {
            base_color: Color::srgb(0.75, 0.6, 0.65),
            metallic: 0.9,
            perceptual_roughness: 0.1,
            ..default()
        },
    ];

    let material_handles: Vec<_> = pale_rose_materials
        .into_iter()
        .map(|mat| materials.add(mat))
        .collect();

    for (i, material) in material_handles.iter().enumerate() {
        let x = (i as f32 - 2.0) * 2.5;
        
        commands.spawn((
            meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
            material.clone(),
            Transform::from_xyz(x, 0.5, -5.0),
        ));

        commands.spawn((
            meshes.add(Sphere::new(0.5).mesh().uv(32, 18)),
            material.clone(),
            Transform::from_xyz(x, 0.5, -3.0),
        ));
    }
}
