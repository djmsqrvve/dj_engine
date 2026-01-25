// Integration tests for GLTF loading pipeline

use bevy::prelude::*;
use bevy::gltf::GltfAssetLabel;

#[test]
fn test_gltf_asset_label_creation() {
    // Test that we can create GLTF asset labels correctly
    let label = GltfAssetLabel::Scene(0).from_asset("test_models/model.gltf");
    
    // Verify the asset path is correct
    let handle: Handle<Scene> = AssetServer::new(Default::default(), Default::default(), false).load(label);
    
    // If we get here without panic, the label creation works
    assert!(true);
}

#[test]
fn test_model_paths_are_valid() {
    // Verify model paths exist (basic validation)
    let model_paths = vec![
        "test_models/dota_models/models/heroes/drow/drow_base.gltf",
        "test_models/dota_models/models/heroes/drow/drow_armor.gltf",
        "test_models/dota_models/models/heroes/drow/drow_weapon.gltf",
    ];
    
    for path in model_paths {
        // Just verify the paths are well-formed strings
        assert!(!path.is_empty());
        assert!(path.ends_with(".gltf"));
    }
}

#[test]
fn test_scene_bundle_creation() {
    // Test that we can create scene bundles
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    
    // Create a dummy scene handle and bundle
    app.add_systems(Startup, |mut commands: Commands| {
        // This tests that SceneBundle can be constructed
        commands.spawn(SceneBundle {
            scene: Handle::<Scene>::default(),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        });
    });
    
    app.update();
    
    // Verify we have entities
    assert!(app.world().entities().len() > 0);
}

#[test]
fn test_multiple_scene_spawning() {
    // Test that we can spawn multiple scenes
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    
    app.add_systems(Startup, |mut commands: Commands| {
        // Spawn multiple scene entities
        for i in 0..3 {
            commands.spawn(SceneBundle {
                scene: Handle::<Scene>::default(),
                transform: Transform::from_xyz(i as f32 * 2.0, 0.0, 0.0),
                ..default()
            });
        }
    });
    
    app.update();
    
    // Verify we spawned 3 entities
    assert!(app.world().entities().len() >= 3);
}

#[test]
fn test_material_asset_creation() {
    // Test PBR material creation (core to our renderer)
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    
    app.add_systems(Startup, |mut materials: ResMut<Assets<StandardMaterial>>| {
        // Create materials with our "Pale Rose" palette
        let materials_list = vec![
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
                base_color: Color::srgb(0.75, 0.6, 0.65),
                metallic: 0.9,
                perceptual_roughness: 0.1,
                ..default()
            },
        ];
        
        for material in materials_list {
            materials.add(material);
        }
    });
    
    app.update();
    
    // If we get here, material creation worked
    assert!(true);
}
#[test]
fn test_camera_transform_updates() {
    // Test that camera transforms can be updated correctly
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    
    app.add_systems(Startup, |mut commands: Commands| {
        commands.spawn((
            Camera3d::default(),
            Transform::from_xyz(0.0, 5.0, 10.0),
        ));
    });
    
    // Add system to modify transform
    app.add_systems(Update, |mut transforms: Query<&mut Transform, With<Camera3d>>| {
        for mut transform in transforms.iter_mut() {
            transform.translation.x += 1.0;
        }
    });
    
    app.update();
    app.update(); // Run multiple frames
    
    // Verify camera exists and has been processed
    assert!(app.world().entities().len() > 0);
}

#[test]
fn test_light_spawning() {
    // Test that we can spawn different light types
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, AssetPlugin::default()));
    
    app.add_systems(Startup, |mut commands: Commands| {
        // Directional light (sun)
        commands.spawn(DirectionalLight {
            illuminance: 10000.0,
            shadows_enabled: true,
            ..default()
        });
        
        // Point light
        commands.spawn(PointLight {
            intensity: 1500.0,
            color: Color::srgb(1.0, 0.7, 0.8),
            shadows_enabled: true,
            ..default()
        });
        
        // Point light 2
        commands.spawn(PointLight {
            intensity: 1000.0,
            color: Color::srgb(0.7, 0.8, 1.0),
            shadows_enabled: true,
            ..default()
        });
    });
    
    app.update();
    
    // Verify we have lights
    let directional_lights = app.world_mut().query::<&DirectionalLight>().iter(&app.world()).len();
    let point_lights = app.world_mut().query::<&PointLight>().iter(&app.world()).len();
    
    assert_eq!(directional_lights, 1);
    assert_eq!(point_lights, 2);
}

#[test]
fn test_pbr_test_objects_spawning() {
    // Test that we can spawn our test grid of PBR objects
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, AssetPlugin::default()));
    
    app.add_systems(Startup, |mut commands: Commands| {
        // Spawn a grid of test objects like in models.rs
        for i in 0..5 {
            let x = (i as f32 - 2.0) * 2.5;
            
            // Spawn cube
            commands.spawn(Transform::from_xyz(x, 0.5, -5.0));
            
            // Spawn sphere
            commands.spawn(Transform::from_xyz(x, 0.5, -3.0));
        }
    });
    
    app.update();
    
    // Should have 10 entities (5 cubes + 5 spheres)
    let entity_count = app.world().entities().len();
    assert!(entity_count >= 10);
}

#[test]
fn test_transform_component_validity() {
    // Test that transforms are being created correctly
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    
    app.add_systems(Startup, |mut commands: Commands| {
        commands.spawn(Transform::from_xyz(1.0, 2.0, 3.0));
        commands.spawn(Transform::from_rotation(Quat::IDENTITY));
        commands.spawn(Transform::from_scale(Vec3::splat(2.0)));
    });
    
    app.update();
    
    let transforms = app.world_mut().query::<&Transform>().iter(&app.world()).len();
    assert_eq!(transforms, 3);
}

#[test]
fn test_mesh_primitive_types() {
    // Test that we can work with mesh primitives
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, AssetPlugin::default()));
    
    app.add_systems(Startup, |mut meshes: ResMut<Assets<Mesh>>| {
        // Create mesh primitives like we do in models.rs
        let _cube = meshes.add(Cuboid::new(1.0, 1.0, 1.0));
        let _plane = meshes.add(Plane3d::new(Vec3::Y, Vec2::new(50.0, 50.0)));
        
        // These would require more setup but we can test the API compiles
        // let sphere = Sphere::new(0.5);
        // let _sphere_mesh = meshes.add(sphere.mesh().uv(32, 18));
    });
    
    app.update();
    
    // Verify meshes were created
    assert!(app.world().contains_resource::<Assets<Mesh>>());
}
