// Diagnostic plugin to debug black screen issue

use bevy::prelude::*;
use bevy::render::camera::RenderTarget;

pub struct DiagnosticPlugin;

impl Plugin for DiagnosticPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_diagnostic_scene)
            .add_systems(Update, (
                log_materials,
                log_lights,
                log_camera_position,
                log_render_resources,
            ));
    }
}

fn spawn_diagnostic_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    error!("╔═══════════════════════════════════════════════════════════════╗");
    error!("║        DIAGNOSTIC SCENE - FORCING VISIBLE OBJECTS             ║");
    error!("╚═══════════════════════════════════════════════════════════════╝");
    
    // Force unlit/emissive materials (no lighting needed)
    let diagnostic_red = materials.add(StandardMaterial {
        base_color: Color::srgb(1.0, 0.0, 0.0),
        emissive: LinearRgba::rgb(100.0, 0.0, 0.0), // Very bright
        unlit: true, // Don't need lighting
        ..default()
    });
    
    let diagnostic_green = materials.add(StandardMaterial {
        base_color: Color::srgb(0.0, 1.0, 0.0),
        emissive: LinearRgba::rgb(0.0, 100.0, 0.0),
        unlit: true,
        ..default()
    });
    
    let diagnostic_blue = materials.add(StandardMaterial {
        base_color: Color::srgb(0.0, 0.0, 1.0),
        emissive: LinearRgba::rgb(0.0, 0.0, 100.0),
        unlit: true,
        ..default()
    });
    
    let diagnostic_yellow = materials.add(StandardMaterial {
        base_color: Color::srgb(1.0, 1.0, 0.0),
        emissive: LinearRgba::rgb(100.0, 100.0, 0.0),
        unlit: true,
        ..default()
    });
    
    // Large cubes at various positions
    error!("Spawning LARGE diagnostic cubes with UNLIT materials...");
    
    commands.spawn((
        meshes.add(Cuboid::new(2.0, 2.0, 2.0)),
        diagnostic_red.clone(),
        Transform::from_xyz(0.0, 2.0, 0.0),
    ));
    error!("  RED cube at (0, 2, 0) - should be VERY visible");
    
    commands.spawn((
        meshes.add(Cuboid::new(2.0, 2.0, 2.0)),
        diagnostic_green.clone(),
        Transform::from_xyz(-5.0, 2.0, 0.0),
    ));
    error!("  GREEN cube at (-5, 2, 0)");
    
    commands.spawn((
        meshes.add(Cuboid::new(2.0, 2.0, 2.0)),
        diagnostic_blue.clone(),
        Transform::from_xyz(5.0, 2.0, 0.0),
    ));
    error!("  BLUE cube at (5, 2, 0)");
    
    commands.spawn((
        meshes.add(Cuboid::new(2.0, 2.0, 2.0)),
        diagnostic_yellow.clone(),
        Transform::from_xyz(0.0, 2.0, -5.0),
    ));
    error!("  YELLOW cube at (0, 2, -5)");
    
    // Ground plane with diagnostic color
    let diagnostic_ground = materials.add(StandardMaterial {
        base_color: Color::srgb(0.8, 0.2, 0.8), // Purple, stands out
        emissive: LinearRgba::rgb(50.0, 10.0, 50.0),
        unlit: true,
        ..default()
    });
    
    error!("Creating DIAGNOSTIC ground plane...");
    commands.spawn((
        meshes.add(Plane3d::new(Vec3::Y, Vec2::new(30.0, 30.0))),
        diagnostic_ground,
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
    error!("  PURPLE ground plane at y=0");
    
    error!("═══════════════════════════════════════════════════════════════");
    error!("DIAGNOSTIC SCENE COMPLETE - all objects use UNLIT materials");
    error!("If you see ANY colors, rendering is working!");
    error!("═══════════════════════════════════════════════════════════════");
}

fn log_materials(
    materials: Query<&Handle<StandardMaterial>>,
    assets: Res<Assets<StandardMaterial>>,
) {
    for _ in materials.iter() {
        // Force material logging
    }
    
    if assets.iter().count() > 0 {
        info!("Materials loaded: {}", assets.iter().count());
        for (id, material) in assets.iter() {
            info!("  Material {:?}: emissive={:?}, unlit={}", 
                  id, material.emissive, material.unlit);
        }
    }
}

fn log_lights(
    directional: Query<&DirectionalLight>,
    point: Query<&PointLight>,
) {
    if directional.iter().len() > 0 {
        info!("Directional lights: {}", directional.iter().len());
    }
    if point.iter().len() > 0 {
        info!("Point lights: {}", point.iter().len());
    }
}

fn log_camera_position(
    cameras: Query<(&Transform, &Camera3d)>,
) {
    for (transform, camera) in cameras.iter() {
        error!("Camera: pos={:?}, forward={:?}, clear_color={:?}", 
               transform.translation, 
               transform.forward(),
               camera.clear_color);
    }
}

fn log_render_resources(
    meshes: Res<Assets<Mesh>>,
    materials: Res<Assets<StandardMaterial>>,
) {
    error!("Render Resources:");
    error!("  Meshes: {}", meshes.iter().count());
    error!("  Materials: {}", materials.iter().count());
    
    // Force API query
    if let Ok(render_adapter) = std::env::var("WGPU_ADAPTER_NAME") {
        error!("  Render Adapter: {}", render_adapter);
    }
}
