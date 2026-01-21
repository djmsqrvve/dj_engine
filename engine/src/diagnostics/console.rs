use bevy::{prelude::*, app::AppExit, diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin}};
use std::{io::{self, Write}, sync::{Arc, Mutex, mpsc::{self, Receiver}}};
use crate::story_graph::GraphExecutor;

/// Resource holding the receiver for console input.
#[derive(Resource)]
struct ConsoleReceiver(Arc<Mutex<Receiver<String>>>);

/// Event fired when a CLI command is entered.
#[derive(Event)]
pub struct ConsoleCommandEvent(pub String);

pub struct ConsolePlugin;

impl Plugin for ConsolePlugin {
    fn build(&self, app: &mut App) {
        let (tx, rx) = mpsc::channel();
        
        // Spawn background thread for stdin
        std::thread::spawn(move || {
            let stdin = io::stdin();
            let mut input = String::new();
            loop {
                // Print prompt
                print!("dj> ");
                let _ = io::stdout().flush();
                
                input.clear();
                if stdin.read_line(&mut input).is_ok() {
                    let cmd = input.trim().to_string();
                    if !cmd.is_empty() {
                        if tx.send(cmd).is_err() {
                            break;
                        }
                    }
                } else {
                    break;
                }
            }
        });

        app.insert_resource(ConsoleReceiver(Arc::new(Mutex::new(rx))))
            .add_event::<ConsoleCommandEvent>()
            .add_systems(Update, listen_for_console_input)
            .add_systems(Update, handle_console_commands);
        
        info!("Console CLI API initialized. Type 'help' in terminal for commands.");
    }
}

fn listen_for_console_input(
    receiver: Res<ConsoleReceiver>,
    mut events: EventWriter<ConsoleCommandEvent>,
) {
    if let Ok(rx) = receiver.0.lock() {
        while let Ok(cmd) = rx.try_recv() {
            events.send(ConsoleCommandEvent(cmd));
        }
    }
}

fn handle_console_commands(
    mut events: EventReader<ConsoleCommandEvent>,
    mut app_exit: EventWriter<AppExit>,
    windows: Query<(Entity, &Window)>,
    entities: Query<Entity>,
    executor: Option<Res<GraphExecutor>>,
    diagnostics: Res<DiagnosticsStore>,
) {
    for event in events.read() {
        let cmd = event.0.to_lowercase();
        let args: Vec<&str> = cmd.split_whitespace().collect();
        
        if args.is_empty() { continue; }
        
        match args[0] {
            "help" => {
                println!("\n--- DJ Engine CLI Help ---");
                println!("help     - Show this help");
                println!("windows  - List open windows and status");
                println!("entities - Show entity count breakdown");
                println!("story    - Show story graph execution state");
                println!("fps      - Show current performance metrics");
                println!("exit     - Close the engine");
                println!("--------------------------\n");
            }
            "windows" => {
                println!("\n--- Active Windows ---");
                let mut found = false;
                for (entity, window) in windows.iter() {
                    found = true;
                    println!(
                        "Entity: {:?}\nTitle: \"{}\"\nResolution: {}x{}\nPosition: {:?}\nFocused: {}\nVisible: {}",
                        entity, window.title, window.width(), window.height(),
                        window.position, window.focused, window.visible
                    );
                }
                if !found { println!("No active windows detected (Headless mode?)"); }
                println!("----------------------\n");
            }
            "entities" => {
                let count = entities.iter().count();
                println!("\nTotal Entities: {}\n", count);
            }
            "story" => {
                println!("\n--- Story Graph Status ---");
                if let Some(exec) = executor.as_ref() {
                    println!("Status: {:?}", exec.status);
                    println!("Active Node: {:?}", exec.current_node);
                    println!("Has Active Graph: {}", exec.active_graph.is_some());
                } else {
                    println!("No GraphExecutor resource found.");
                }
                println!("--------------------------\n");
            }
            "fps" => {
                let fps = diagnostics
                    .get(&FrameTimeDiagnosticsPlugin::FPS)
                    .and_then(|diag| diag.smoothed())
                    .unwrap_or(0.0);
                let ft = diagnostics
                    .get(&FrameTimeDiagnosticsPlugin::FRAME_TIME)
                    .and_then(|diag| diag.smoothed())
                    .unwrap_or(0.0);
                println!("\nFPS: {:.1}\nFrame Time: {:.2}ms\n", fps, ft);
            }
            "exit" | "quit" => {
                println!("Exiting engine...");
                app_exit.send(AppExit::Success);
            }
            _ => {
                println!("Unknown command: '{}'. Type 'help' for available commands.", args[0]);
            }
        }
        
        // Final prompt for the next input
        let _ = io::stdout().flush();
    }
}
