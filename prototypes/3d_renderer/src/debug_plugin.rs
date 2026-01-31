use bevy::prelude::*;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_debug_grid)
            .add_systems(Update, log_entity_count);
    }
}

fn spawn_debug_grid(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    info!("=== SPAWNING DEBUG VISUALIZATION ===");
    
    // Create a bright white material
    let debug_material = materials.add(StandardMaterial {
        base_color: Color::srgb(1.0, 1.0, 1.0),
        ..default()
    });
    
    // Spawn a 5x5 grid of cubes at various heights
    for x in -2..=2 {
        for z in -2..=2 {
            let height = (x * x + z * z) as f32 * 0.5;
            
            commands.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.3, 0.3, 0.3))),
                MeshMaterial3d(debug_material.clone()),
                Transform::from_xyz(x as f32 * 2.0, height + 1.0, z as f32 * 2.0),
            ));
            
            info!("Spawned debug cube at ({}, {}, {})", x as f32 * 2.0, height + 1.0, z as f32 * 2.0);
        }
    }
    
    // Spawn a bright reference point at origin
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(0.5, 0.5, 0.5))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(1.0, 0.0, 0.0),
            ..default()
        })),
        Transform::from_xyz(0.0, 0.5, 0.0),
    ));
    
    info!("Spawned red reference cube at origin");
    info!("=== DEBUG VISUALIZATION COMPLETE ===");
}

fn log_entity_count(
    query: Query<&Transform, With<Mesh3d>>,
    time: Res<Time>,
) {
    if time.elapsed_secs() > 1.0 && time.elapsed_secs() < 1.1 {
        info!("Total entities with meshes: {}", query.iter().len());
    }
}
