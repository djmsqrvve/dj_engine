use bevy::prelude::*;
use dj_engine::prelude::*;
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
    mut commands: Commands,
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

fn rpg_output_system(mut events: EventReader<StoryFlowEvent>) {
    for event in events.read() {
        match event {
            StoryFlowEvent::ShowDialogue { speaker, text, .. } => {
                let content = text.get("en").unwrap_or(&"???".to_string());
                println!("\n[{}] {}", speaker.as_degug_str(), content);
                println!("(Press Enter to continue)");
            }
            StoryFlowEvent::ShowChoices { prompt, options } => {
                let content = prompt.get("en").unwrap_or(&"???".to_string());
                println!("\n? {}", content);
                for (i, opt) in options.iter().enumerate() {
                    let opt_text = opt.get("en").unwrap_or(&"???".to_string());
                    println!("  {}. {}", i + 1, opt_text);
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
    mut console_events: EventReader<dj_engine::diagnostics::console::ConsoleCommandEvent>,
    mut story_events: EventWriter<StoryInputEvent>,
) {
    for event in console_events.read() {
        let input = event.0.trim();
        
        // Check if input is a number
        if let Ok(index) = input.parse::<usize>() {
            if index > 0 {
                 println!("> Selected option {}", index);
                 story_events.send(StoryInputEvent::SelectChoice(index - 1));
                 continue;
            }
        }

        // Check for advance commands
        match input.to_lowercase().as_str() {
            "" | "next" | "n" => {
                story_events.send(StoryInputEvent::Advance);
            }
            "quit" | "exit" => {
                println!("Quitting...");
                std::process::exit(0);
            }
            _ => {
                // If not a number or command, treat empty enter as advance too?
                // The console plugin filters empty strings usually, but let's see.
            }
        }
    }
}

trait DebugStr {
    fn as_degug_str(&self) -> String;
}

impl DebugStr for Option<String> {
    fn as_degug_str(&self) -> String {
        self.clone().unwrap_or("Narrator".to_string())
    }
}
