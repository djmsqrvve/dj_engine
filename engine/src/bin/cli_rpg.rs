use bevy::prelude::*;
use dj_engine::prelude::*;
use dj_engine::diagnostics::console::ConsoleCommandEvent;
use std::path::Path;

fn main() {
    println!("--- DJ Engine CLI RPG Runner ---");
    println!("Initializing...");

    App::new()
        .add_plugins(MinimalPlugins)
        .add_plugins(DiagnosticsPlugin) // Includes ConsolePlugin
        .add_plugins(StoryGraphPlugin)
        .add_systems(Startup, setup_rpg)
        .add_systems(Update, (rpg_output_system, rpg_input_system))
        .run();
}

fn setup_rpg(
    // mut commands: Commands, // Unused
    mut library: ResMut<StoryGraphLibrary>,
    mut executor: ResMut<GraphExecutor>,
) {
    // 1. Load the specific test game paths
    // In a real app, this would be dynamic, but for verification we hardcode relative to workspace
    let game_root = Path::new("games/cli_test_game");
    let graph_path = game_root.join("story_graphs/test_game.json");

    println!("Loading story graph from: {:?}", graph_path);

    match load_story_graph(&graph_path) {
        Ok(data) => {
            println!("Graph '{}' loaded successfully.", data.id);
            
            // Load and start the graph
            executor.load_from_data(&data, &mut library);
            
            println!("Story started! type 'next' to advance or numbers to choose.");
        }
        Err(e) => {
            error!("Failed to load story graph: {}", e);
            println!("CRITICAL ERROR: Could not load story graph. Check paths.");
        }
    }
}

fn rpg_output_system(mut events: MessageReader<StoryFlowEvent>) {
    for event in events.read() {
        match event {
            StoryFlowEvent::ShowDialogue { speaker, text, .. } => {
                println!("\n[{}] {}", speaker, text);
                println!("(Press Enter to continue)");
            }
            StoryFlowEvent::ShowChoices { prompt, options } => {
                println!("\n? {}", prompt);
                for (i, opt) in options.iter().enumerate() {
                    println!("  {}. {}", i + 1, opt);
                }
            }
            StoryFlowEvent::GraphComplete => {
                println!("\n--- THE END ---");
                println!("Story graph execution completed.");
                std::process::exit(0);
            }
            StoryFlowEvent::CameraControl { .. } => {
                println!("[DEBUG] Camera Move Triggered");
            }
            StoryFlowEvent::TimeControl { .. } => {
                println!("[DEBUG] Time Control Triggered");
            }
        }
    }
}

fn rpg_input_system(
    mut console_events: MessageReader<ConsoleCommandEvent>,
    mut story_events: MessageWriter<StoryInputEvent>,
) {
    for event in console_events.read() {
        let cmd = event.0.trim();
        if cmd == "next" || cmd.is_empty() {
            story_events.write(StoryInputEvent::Advance);
        } else if let Ok(choice) = cmd.parse::<usize>() {
            story_events.write(StoryInputEvent::SelectChoice(choice.saturating_sub(1)));
        }
    }
}


