// Simple diagnostic main.rs - fixed API issues

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy 3D Renderer - DIAGNOSTIC".into(),
                resolution: (1920.0, 1080.0).into(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, spawn_diagnostic_scene)
        .add_systems(Update, (
            print_info,
            keyboard_exit,
        ))
        .run();
}

fn spawn_diagnostic_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Print to BOTH stdout (for capture) and stderr (always visible)
    eprintln!("═══════════════════════════════════════════════════════════════");
    eprintln!("          DIAGNOSTIC SCENE - FORCING VISIBLE OBJECTS");
    eprintln!("═══════════════════════════════════════════════════════════════");
    eprintln!("");
    eprintln!("If you see NO COLORS, the window is not rendering at all.");
    eprintln!("If you see ANY colors, the issue is with lighting/visibility.");
    eprintln!("");
    
    // Use bevy logging as well
    error!("DIAGNOSTIC SCENE SPAWNING...");

    // Very bright unlit materials
    let red = materials.add(StandardMaterial {
        base_color: Color::srgb(1.0, 0.0, 0.0),
        emissive: LinearRgba::rgb(10.0, 0.0, 0.0),
        unlit: true,
        ..default()
    });
    
    let green = materials.add(StandardMaterial {
        base_color: Color::srgb(0.0, 1.0, 0.0),
        emissive: LinearRgba::rgb(0.0, 10.0, 0.0),
        unlit: true,
        ..default()
    });
    
    let blue = materials.add(StandardMaterial {
        base_color: Color::srgb(0.0, 0.0, 1.0),
        emissive: LinearRgba::rgb(0.0, 0.0, 10.0),
        unlit: true,
        ..default()
    });
    
    let yellow = materials.add(StandardMaterial {
        base_color: Color::srgb(1.0, 1.0, 0.0),
        emissive: LinearRgba::rgb(10.0, 10.0, 0.0),
        unlit: true,
        ..default()
    });

    // Large cubes
    error!("Spawning RED cube at (0, 2, 0) - VERY BRIGHT");
    commands.spawn((meshes.add(Cuboid::new(2.0, 2.0, 2.0)), red, Transform::from_xyz(0.0, 2.0, 0.0)));
    
    error!("Spawning GREEN cube at (-5, 2, 0)");
    commands.spawn((meshes.add(Cuboid::new(2.0, 2.0, 2.0)), green, Transform::from_xyz(-5.0, 2.0, 0.0)));
    
    error!("Spawning BLUE cube at (5, 2, 0)");
    commands.spawn((meshes.add(Cuboid::new(2.0, 2.0, 2.0)), blue, Transform::from_xyz(5.0, 2.0, 0.0)));
    
    error!("Spawning YELLOW cube at (0, 2, -5)");
    commands.spawn((meshes.add(Cuboid::new(2.0, 2.0, 2.0)), yellow, Transform::from_xyz(0.0, 2.0, -5.0)));

    // Ground
    let ground = materials.add(StandardMaterial {
        base_color: Color::srgb(0.8, 0.2, 0.8),
        emissive: LinearRgba::rgb(5.0, 1.0, 5.0),
        unlit: true,
        ..default()
    });
    commands.spawn((meshes.add(Plane3d::new(Vec3::Y, Vec2::new(30.0, 30.0))), ground, Transform::from_xyz(0.0, 0.0, 0.0)));
    error!("Spawning PURPLE ground plane at y=0");

    // Camera with bright clear color
    commands.spawn((Camera3d::default(), Transform::from_xyz(0.0, 10.0, 15.0).looking_at(Vec3::ZERO, Vec3::Y), Camera {
        clear_color: ClearColorConfig::Custom(Color::srgb(0.0, 1.0, 1.0)), // Cyan
        ..default()
    }));
    error!("Camera at (0,10,15), clear_color=CYAN (very visible)");
    
    error!("DIAGNOSTIC SCENE COMPLETE - all objects are UNLIT and BRIGHT");
}

fn print_info(
    mesh_query: Query<&Handle<Mesh>>,
    time: Res<Time>,
) {
    if (time.elapsed_seconds() * 10.0) as i32 % 50 == 0 {
        let count = mesh_query.iter().len();
        error!("ENTITY COUNT: {} meshes in scene", count);
    }
}

fn keyboard_exit(
    keys: Res<ButtonInput<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        eprintln!("ESCAPE pressed - check if you see ANY colors in the window!");
        std::process::exit(0);
    }
    if keys.just_pressed(KeyCode::Space) {
        eprintln!("SPACE pressed - looking for visual output...");
    }
}

// Use this to create diagnostic version:
// cp src/simple_main_fixed.rs src/main.rs
// Run: cargo run --release 2>&1 | tee diagnostic.log
