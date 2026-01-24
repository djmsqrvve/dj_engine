use bevy::prelude::*;
use bevy::window::WindowResolution;
use dj_engine::prelude::*;

mod assets;
mod battle;
mod dialogue;
mod hamster;
mod hud;
mod overworld;
mod scripting;
mod state;
mod story;
mod title;
mod types;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let editor_mode = args.contains(&"--editor".to_string());
    
    // Parse Resolution / Profile
    let mut width = if editor_mode { 1920.0 } else { 1024.0 };
    let mut height = if editor_mode { 1080.0 } else { 768.0 };
    
    for i in 0..args.len() {
        if args[i] == "--profile" && i + 1 < args.len() {
            match args[i+1].as_str() {
                "half" => {
                    width = 1280.0;
                    height = 1440.0;
                }
                "720p" => {
                    width = 1280.0;
                    height = 720.0;
                }
                "1080p" => {
                    width = 1920.0;
                    height = 1080.0;
                }
                _ => warn!("Unknown profile: {}", args[i+1]),
            }
        }
        if args[i] == "--width" && i + 1 < args.len() {
            if let Ok(w) = args[i+1].parse::<f32>() { width = w; }
        }
        if args[i] == "--height" && i + 1 < args.len() {
            if let Ok(h) = args[i+1].parse::<f32>() { height = h; }
        }
    }

    let mut app = App::new();

    app.add_plugins(
        DefaultPlugins
            .set(ImagePlugin::default_nearest()) // Pixel art friendly
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: if editor_mode { "DJ Engine - Helix Bridge Editor" } else { "DJ Engine - Helix Bridge" }.into(),
                    resolution: WindowResolution::new(width, height)
                        .with_scale_factor_override(1.0),
                    position: WindowPosition::Centered(MonitorSelection::Primary),
                    present_mode: bevy::window::PresentMode::AutoVsync,
                    ..default()
                }),
                ..default()
            }),
    )
    .add_plugins(DJEnginePlugin::default())
    .add_plugins(scripting::GameScriptingPlugin)
    .init_state::<state::GameState>()
    .add_plugins(title::TitlePlugin)
    .add_plugins(story::StoryPlugin)
    .add_plugins(hamster::HamsterPlugin)
    .add_plugins(overworld::OverworldPlugin)
    .add_plugins(hud::HudPlugin)
    .add_plugins(dialogue::DialoguePlugin)
    .add_plugins(battle::BattlePlugin)
    .add_plugins(assets::GameAssetsPlugin);

    if editor_mode {
        // EDITOR MODE:
        // 1. Add Editor Plugin
        // 2. Start in GameState::Editor (so no game systems run yet)
        // 3. Sync EditorState -> GameState
        app.add_plugins(dj_engine::editor::EditorPlugin);
        
        // Ensure we start in Editor state
        app.insert_state(state::GameState::Editor);
        
        // Sync system
        app.add_systems(Update, sync_editor_state);
    }

    app.run();
}

/// Syncs the generic Engine EditorState with the specific GameState
fn sync_editor_state(
    editor_state: Res<State<dj_engine::editor::EditorState>>,
    game_state: Res<State<state::GameState>>,
    mut next_game_state: ResMut<NextState<state::GameState>>,
) {
    use dj_engine::editor::EditorState;
    use state::GameState;

    if editor_state.is_changed() {
        match editor_state.get() {
            EditorState::Editor => {
                if *game_state.get() != GameState::Editor {
                    next_game_state.set(GameState::Editor);
                }
            }
            EditorState::Playing => {
                if *game_state.get() == GameState::Editor {
                    // Start the game!
                    next_game_state.set(GameState::TitleScreen);
                }
            }
        }
    }
}
