use bevy::prelude::*;
use bevy::window::WindowResolution;
use dj_engine::prelude::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let editor_mode = args.contains(&"--editor".to_string());

    let mut app = App::new();

    app.add_plugins(
        DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "New Horizon - DJ Engine".into(),
                    resolution: WindowResolution::new(1280.0, 720.0),
                    position: WindowPosition::Centered(MonitorSelection::Primary),
                    present_mode: bevy::window::PresentMode::AutoVsync,
                    ..default()
                }),
                ..default()
            }),
    )
    .add_plugins(DJEnginePlugin::default())
    .add_systems(Startup, setup_game);

    if editor_mode {
        app.add_plugins(dj_engine::editor::EditorPlugin);
    }

    app.run();
}

fn setup_game(mut commands: Commands) {
    // Basic setup will go here
    info!("New Horizon initialized!");
}
