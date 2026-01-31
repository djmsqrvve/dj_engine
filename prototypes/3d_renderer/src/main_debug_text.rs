// Debug version with on-screen text and XYZ axes to verify rendering is working
// This helps diagnose if it's a GLTF issue or a general rendering issue

use bevy::prelude::*;
use bevy::color::palettes::basic::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Debug Text & XYZ Axes - Verify Rendering".into(),
                resolution: (1920.0, 1080.0).into(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup_debug_scene)
        .add_systems(Update, (
            update_debug_text,
            draw_xyz_axes,
            draw_grid,
            keyboard_input,
            rotate_camera,
        ))
        .run();
}

fn setup_debug_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    eprintln!("═══════════════════════════════════════════════════════════════");
    eprintln!("  DEBUG SCENE: Text + XYZ Axes + Diagnostic Cubes");
    eprintln!("═══════════════════════════════════════════════════════════════");

    // Camera with bright clear color
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(10.0, 8.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        Camera {
            clear_color: ClearColorConfig::Custom(Color::srgb(0.1, 0.1, 0.15)), // Dark blue-gray background
            ..default()
        },
    ));

    // ==== 2D TEXT OVERLAYS ====
    // These will appear on screen if ANY rendering is working
    // Using default font (no font handle needed)
    
    // Title text at top
    commands.spawn((
        Text2d::new("DEBUG RENDER TEST"),
        TextFont {
            font_size: 48.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Transform::from_xyz(0.0, 450.0, 0.0), // Top center
    ));

    // Status text - will be updated
    commands.spawn((
        Text2d::new("Status: Initializing..."),
        TextFont {
            font_size: 32.0,
            ..default()
        },
        TextColor(Color::srgb(0.0, 1.0, 0.0)), // Bright green
        Transform::from_xyz(0.0, 380.0, 0.0),
        DebugStatusText,
    ));

    // Instructions
    commands.spawn((
        Text2d::new("Controls: SPACE = pause rotation | ESC = exit"),
        TextFont {
            font_size: 24.0,
            ..default()
        },
        TextColor(Color::srgb(0.8, 0.8, 0.8)),
        Transform::from_xyz(0.0, -480.0, 0.0), // Bottom center
    ));

    // Corner markers to verify screen edges
    spawn_corner_text(&mut commands, "[RED]", -850.0, 480.0, Color::from(RED));
    spawn_corner_text(&mut commands, "[GREEN]", 850.0, 480.0, Color::from(GREEN));
    spawn_corner_text(&mut commands, "[BLUE]", -850.0, -480.0, Color::from(BLUE));
    spawn_corner_text(&mut commands, "[YELLOW]", 850.0, -480.0, Color::from(YELLOW));

    // Coordinate labels
    spawn_corner_text(&mut commands, "+X is RIGHT", 600.0, 400.0, Color::from(RED));
    spawn_corner_text(&mut commands, "+Y is UP", 600.0, 360.0, Color::from(GREEN));
    spawn_corner_text(&mut commands, "+Z is TOWARD CAMERA", 600.0, 320.0, Color::from(BLUE));

    // ==== 3D DIAGNOSTIC CUBES ====
    // Large unlit cubes that should be visible if 3D rendering works
    
    let red_material = materials.add(StandardMaterial {
        base_color: Color::srgb(1.0, 0.2, 0.2),
        emissive: LinearRgba::rgb(2.0, 0.0, 0.0),
        unlit: true,
        ..default()
    });
    
    let green_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.2, 1.0, 0.2),
        emissive: LinearRgba::rgb(0.0, 2.0, 0.0),
        unlit: true,
        ..default()
    });
    
    let blue_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.2, 0.2, 1.0),
        emissive: LinearRgba::rgb(0.0, 0.0, 2.0),
        unlit: true,
        ..default()
    });

    // X-axis marker (Red) - extending to the right
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(4.0, 0.3, 0.3))),
        MeshMaterial3d(red_material.clone()),
        Transform::from_xyz(2.0, 0.5, 0.0),
    ));

    // Y-axis marker (Green) - extending up
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(0.3, 4.0, 0.3))),
        MeshMaterial3d(green_material.clone()),
        Transform::from_xyz(0.0, 2.5, 0.0),
    ));

    // Z-axis marker (Blue) - extending toward/away from camera
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(0.3, 0.3, 4.0))),
        MeshMaterial3d(blue_material.clone()),
        Transform::from_xyz(0.0, 0.5, 2.0),
    ));

    // Origin cube
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(0.8, 0.8, 0.8))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(1.0, 1.0, 0.0),
            emissive: LinearRgba::rgb(2.0, 2.0, 0.0),
            unlit: true,
            ..default()
        })),
        Transform::from_xyz(0.0, 0.4, 0.0),
    ));

    // Ground plane
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::new(Vec3::Y, Vec2::new(50.0, 50.0)))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.15, 0.15, 0.2),
            unlit: true,
            ..default()
        })),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    // Directional light (even though materials are unlit, for completeness)
    commands.spawn((
        DirectionalLight {
            illuminance: 10000.0,
            ..default()
        },
        Transform::from_xyz(10.0, 20.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    eprintln!("Scene setup complete:");
    eprintln!("  - 2D Text overlays at corners and center");
    eprintln!("  - XYZ axis cubes (R=+X, G=+Y, B=+Z)");
    eprintln!("  - Yellow origin cube at (0,0,0)");
    eprintln!("  - Ground plane");
    eprintln!("");
    eprintln!("IF YOU SEE THIS MESSAGE BUT NO WINDOW:");
    eprintln!("  → The app is running but rendering is completely broken");
    eprintln!("IF YOU SEE TEXT BUT NO 3D OBJECTS:");
    eprintln!("  → 2D rendering works, 3D rendering has issues");
    eprintln!("IF YOU SEE EVERYTHING:");
    eprintln!("  → Rendering is working! GLTF loading may be the issue");
    eprintln!("═══════════════════════════════════════════════════════════════");
}

fn spawn_corner_text(
    commands: &mut Commands,
    text: &str,
    x: f32,
    y: f32,
    color: Color,
) {
    commands.spawn((
        Text2d::new(text),
        TextFont {
            font_size: 24.0,
            ..default()
        },
        TextColor(color),
        Transform::from_xyz(x, y, 0.0),
    ));
}

#[derive(Component)]
struct DebugStatusText;

fn update_debug_text(
    mut query: Query<&mut Text2d, With<DebugStatusText>>,
    time: Res<Time>,
) {
    for mut text in query.iter_mut() {
        let elapsed = time.elapsed_secs();
        let dots = ((elapsed * 2.0) as usize % 4) + 1;
        let dots_str = "█".repeat(dots);
        text.0 = format!("Running {} | Time: {:.1}s", dots_str, elapsed);
    }
}

fn draw_xyz_axes(
    mut gizmos: Gizmos,
) {
    // Draw XYZ axes using gizmos - extending in both directions
    // X axis - Red
    gizmos.line(
        Vec3::new(-50.0, 0.01, 0.0),
        Vec3::new(50.0, 0.01, 0.0),
        Color::from(RED),
    );
    // Y axis - Green
    gizmos.line(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 10.0, 0.0),
        Color::from(GREEN),
    );
    // Z axis - Blue
    gizmos.line(
        Vec3::new(0.0, 0.01, -50.0),
        Vec3::new(0.0, 0.01, 50.0),
        Color::from(BLUE),
    );

    // Draw arrow heads
    gizmos.line(Vec3::new(5.0, 0.0, 0.0), Vec3::new(4.5, 0.3, 0.0), Color::from(RED));
    gizmos.line(Vec3::new(5.0, 0.0, 0.0), Vec3::new(4.5, -0.3, 0.0), Color::from(RED));
    
    gizmos.line(Vec3::new(0.0, 5.0, 0.0), Vec3::new(0.3, 4.5, 0.0), Color::from(GREEN));
    gizmos.line(Vec3::new(0.0, 5.0, 0.0), Vec3::new(-0.3, 4.5, 0.0), Color::from(GREEN));
    
    gizmos.line(Vec3::new(0.0, 0.0, 5.0), Vec3::new(0.0, 0.3, 4.5), Color::from(BLUE));
    gizmos.line(Vec3::new(0.0, 0.0, 5.0), Vec3::new(0.0, -0.3, 4.5), Color::from(BLUE));
}

fn draw_grid(
    mut gizmos: Gizmos,
) {
    let grid_size = 20;
    let grid_spacing = 2.0;
    let grid_color = Color::srgb(0.2, 0.2, 0.25);

    for i in -grid_size..=grid_size {
        let pos = i as f32 * grid_spacing;
        // Lines along X
        gizmos.line(
            Vec3::new(-grid_size as f32 * grid_spacing, 0.0, pos),
            Vec3::new(grid_size as f32 * grid_spacing, 0.0, pos),
            grid_color,
        );
        // Lines along Z
        gizmos.line(
            Vec3::new(pos, 0.0, -grid_size as f32 * grid_spacing),
            Vec3::new(pos, 0.0, grid_size as f32 * grid_spacing),
            grid_color,
        );
    }
}

#[derive(Resource, Default)]
struct RotationState {
    paused: bool,
}

fn rotate_camera(
    time: Res<Time>,
    mut cameras: Query<&mut Transform, With<Camera3d>>,
    state: Res<RotationState>,
    mut angle: Local<f32>,
) {
    if state.paused {
        return;
    }
    
    // Slowly rotate camera around the scene
    *angle += time.delta_secs() * 0.3;
    let radius = 15.0;
    let x = radius * angle.cos();
    let z = radius * angle.sin();
    
    for mut transform in cameras.iter_mut() {
        transform.translation.x = x;
        transform.translation.z = z;
        transform.look_at(Vec3::ZERO, Vec3::Y);
    }
}

fn keyboard_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut exit: ResMut<Events<bevy::app::AppExit>>,
    mut state: ResMut<RotationState>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        eprintln!("ESC pressed - exiting");
        exit.send(bevy::app::AppExit::Success);
    }

    if keys.just_pressed(KeyCode::Space) {
        state.paused = !state.paused;
        eprintln!("SPACE pressed - rotation {}", if state.paused { "paused" } else { "resumed" });
    }
}
