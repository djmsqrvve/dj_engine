// GLTF visibility verification - place in src/plugins/models.rs

use bevy::prelude::*;
use bevy::gltf::GltfAssetLabel;

pub struct ModelVisibilityPlugin;

impl Plugin for ModelVisibilityPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (load_and_verify_drow, setup_camera_for_model))
            .add_systems(Update, check_model_bounds);
    }
}

#[derive(Resource)]
struct DrowModel(Handle<Scene>);

fn load_and_verify_drow(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    info!("Loading Drow Ranger model...");
    
    // Load the Drow model using GLTF asset label
    let drow_scene = asset_server.load(
        GltfAssetLabel::Scene(0).from_asset("test_models/dota_models/models/heroes/drow/drow_base.gltf")
    );
    
    commands.insert_resource(DrowModel(drow_scene.clone()));
    
    // Spawn at origin with visibility
    commands.spawn((
        SceneRoot(drow_scene),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
    
    info!("Drow model spawned at origin (0,0,0)");
}

fn setup_camera_for_model(mut commands: Commands) {
    // Position camera to definitely see origin
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(5.0, 5.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        Camera {
            clear_color: ClearColorConfig::Custom(Color::srgb(0.1, 0.1, 0.1)), // Dark background
            ..default()
        },
    ));
    
    info!("Camera positioned at (5,5,10) looking at origin for GLTF visibility");
}

fn check_model_bounds(
    query: Query<(&Transform, &Handle<Mesh>), Without<Camera3d>>,
    meshes: Res<Assets<Mesh>>,
) {
    for (transform, handle) in query.iter() {
        if let Some(mesh) = meshes.get(handle) {
            if let Some(aabb) = mesh.compute_aabb() {
                info!("Model bounds: min={:?}, max={:?}, center={:?}, size={:?}",
                      aabb.min, aabb.max, aabb.center, aabb.half_extents * 2.0);
            }
        }
    }
}
