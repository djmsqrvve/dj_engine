// Simple main.rs that directly spawns visible diagnostic objects
// Use this instead of src/main.rs to debug black screen

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy 3D Rendering Sandbox - DIAGNOSTIC MODE".into(),
                resolution: (1920.0, 1080.0).into(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, simple_diagnostic_scene)
        .add_systems(Update, 
            (print_entity_count, move_camera, keyboard_input)
        )
        .run();
}

fn simple_diagnostic_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Force error logging to terminal
    eprintln!("═══════════════════════════════════════════════════════════════");
    eprintln!("          DIAGNOSTIC SCENE - FORCING VISIBLE OBJECTS             ");
    eprintln!("═══════════════════════════════════════════════════════════════");
    eprintln!("");
    eprintln!("Spawning LARGE diagnostic cubes with UNLIT materials...");
    eprintln!("If you see NO COLORS, the window is not rendering at all.");
    eprintln!("If you see ANY colors, the issue is with lighting/visibility.");
    eprintln!("");

    // Create 4 extremely visible diagnostic cubes with unlit materials
    let diagnostic_red = materials.add(StandardMaterial {
        base_color: Color::srgb(1.0, 0.0, 0.0),
        emissive: LinearRgba::rgb(10.0, 0.0, 0.0),
        unlit: true,
        ..default()
    });
    
    let diagnostic_green = materials.add(StandardMaterial {
        base_color: Color::srgb(0.0, 1.0, 0.0),
        emissive: LinearRgba::rgb(0.0, 10.0, 0.0),
        unlit: true,
        ..default()
    });
    
    let diagnostic_blue = materials.add(StandardMaterial {
        base_color: Color::srgb(0.0, 0.0, 1.0),
        emissive: LinearRgba::rgb(0.0, 0.0, 10.0),
        unlit: true,
        ..default()
    });
    
    let diagnostic_yellow = materials.add(StandardMaterial {
        base_color: Color::srgb(1.0, 1.0, 0.0),
        emissive: LinearRgba::rgb(10.0, 10.0, 0.0),
        unlit: true,
        ..default()
    });

    // Large cubes (2x2x2 units)
    eprintln!("  RED cube at (0, 2, 0) - SIZE 2x2x2");
    commands.spawn((
        meshes.add(Cuboid::new(2.0, 2.0, 2.0)),
        diagnostic_red,
        Transform::from_xyz(0.0, 2.0, 0.0),
    ));
    
    eprintln!("  GREEN cube at (-5, 2, 0) - SIZE 2x2x2");
    commands.spawn((
        meshes.add(Cuboid::new(2.0, 2.0, 2.0)),
        diagnostic_green,
        Transform::from_xyz(-5.0, 2.0, 0.0),
    ));
    
    eprintln!("  BLUE cube at (5, 2, 0) - SIZE 2x2x2");
    commands.spawn((
        meshes.add(Cuboid::new(2.0, 2.0, 2.0)),
        diagnostic_blue,
        Transform::from_xyz(5.0, 2.0, 0.0),
    ));
    
    eprintln!("  YELLOW cube at (0, 2, -5) - SIZE 2x2x2");
    commands.spawn((
        meshes.add(Cuboid::new(2.0, 2.0, 2.0)),
        diagnostic_yellow,
        Transform::from_xyz(0.0, 2.0, -5.0),
    ));

    // Ground plane
    let diagnostic_ground = materials.add(StandardMaterial {
        base_color: Color::srgb(0.8, 0.2, 0.8),
        emissive: LinearRgba::rgb(5.0, 1.0, 5.0),
        unlit: true,
        ..default()
    });
    
    eprintln!("  PURPLE ground plane at y=0 - SIZE 30x30");
    commands.spawn((
        meshes.add(Plane3d::new(Vec3::Y, Vec2::new(30.0, 30.0))),
        diagnostic_ground,
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    eprintln!("");
    eprintln!("═══════════════════════════════════════════════════════════════");
    eprintln!("DIAGNOSTIC SCENE COMPLETE - all objects use UNLIT materials");
    eprintln!("═══════════════════════════════════════════════════════════════");

    // Camera with extreme clear color (bright cyan)
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 10.0, 15.0).looking_at(Vec3::ZERO, Vec3::Y),
        Camera {
            clear_color: ClearColorConfig::Custom(Color::srgb(0.0, 1.0, 1.0)),
            ..default()
        },
    ));
    
    eprintln!("Camera: pos=(0,10,15), clear_color=CYAN (very visible)");
}

fn print_entity_count(
    mesh_query: Query<&Handle<Mesh>>,
    time: Res<Time>,
) {
    if time.elapsed_seconds() > 2.0 && time.elapsed_seconds() < 2.1 {
        eprintln!("═══════════════════════════════════════════════════════════════");
        eprintln!("ENTITY COUNT AFTER 2 SECONDS: {}", mesh_query.iter().len());
        eprintln!("═══════════════════════════════════════════════════════════════");
    }
}

fn move_camera(
    time: Res<Time>,
    mut cameras: Query<(&mut Transform, &Camera3d)>,
) {
    for (mut transform, _camera) in cameras.iter_mut() {
        let x = time.elapsed_seconds().sin() * 2.0;
        transform.translation.x = x;
        transform.look_at(Vec3::ZERO, Vec3::Y);
    }
}

fn keyboard_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut exit: ResMut<bevy::app::AppExit>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        eprintln!("ESCAPE pressed - exiting");
        exit.set(bevy::app::AppExit::Success);
    }
    
    if keys.just_pressed(KeyCode::Space) {
        eprintln!("SPACE pressed - taking screenshot of current frame");
        eprintln!("(Note: Screenshot functionality would need additional code)");
    }
}

// Usage: Rename this file to src/main.rs to use the diagnostic scene
//        Keep the original as src/main_original.rs for backup
