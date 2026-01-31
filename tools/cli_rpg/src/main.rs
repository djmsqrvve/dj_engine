//! Standalone CLI Text-Based RPG Runner
//! 
//! This is a minimal implementation that parses StoryGraph JSON files
//! and runs them in a terminal without needing the full engine.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::{self, Write};
use std::path::Path;

// ============================================================================
// Minimal Type Definitions (duplicated from engine/src/data/story.rs)
// ============================================================================

pub type LocalizedString = HashMap<String, String>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum StoryNodeVariant {
    Start(StartNodeData),
    Dialogue(DialogueNodeData),
    Choice(ChoiceNodeData),
    SetFlag(SetFlagNodeData),
    Branch(BranchNodeData),
    Battle(BattleNodeData),
    SubGraph(SubGraphNodeData),
    End(EndNodeData),
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetFlagNodeData {
    pub flag: String,
    pub value: serde_json::Value, // Simplified
    pub next_node_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BranchNodeData {
    pub condition_flag: Option<String>, // Simplification for CLI
    pub true_node_id: Option<String>,
    pub false_node_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BattleNodeData {
    pub enemy_id: String,
    pub win_node_id: Option<String>,
    pub loss_node_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubGraphNodeData {
    pub graph_id: String,
    pub next_node_id: Option<String>,
}


#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StartNodeData {
    #[serde(default)]
    pub next_node_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DialogueNodeData {
    pub speaker_id: String,
    #[serde(default)]
    pub text: LocalizedString,
    #[serde(default)]
    pub next_node_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChoiceNodeData {
    #[serde(default)]
    pub prompt: LocalizedString,
    pub options: Vec<ChoiceOption>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChoiceOption {
    pub id: String,
    pub text: LocalizedString,
    pub target_node_id: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EndNodeData {
    #[serde(default)]
    pub end_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoryNodeData {
    pub id: String,
    #[serde(rename = "type")]
    pub node_type: String,
    pub data: StoryNodeVariant,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoryGraphData {
    pub id: String,
    pub name: String,
    pub root_node_id: String,
    pub nodes: Vec<StoryNodeData>,
}

// ============================================================================
// Game State
// ============================================================================

struct GameState {
    graph: StoryGraphData,
    current_node_id: String,
    flags: HashMap<String, serde_json::Value>,
    running: bool,
    auto_mode: bool,
}

impl GameState {
    fn new(graph: StoryGraphData, auto_mode: bool) -> Self {
        let start = graph.root_node_id.clone();
        Self {
            graph,
            current_node_id: start,
            flags: HashMap::new(),
            running: true,
            auto_mode,
        }
    }

    fn find_node(&self, id: &str) -> Option<&StoryNodeData> {
        self.graph.nodes.iter().find(|n| n.id == id)
    }

    fn get_text(localized: &LocalizedString) -> &str {
        localized.get("en").map(|s| s.as_str()).unwrap_or("???")
    }

    fn process_current_node(&mut self) -> bool {
        let node = match self.find_node(&self.current_node_id.clone()) {
            Some(n) => n.clone(),
            None => {
                println!("\n[ERROR] Node not found: {}", self.current_node_id);
                self.running = false;
                return false;
            }
        };

        match &node.data {
            StoryNodeVariant::Start(data) => {
                println!("\n=== {} ===", self.graph.name);
                if let Some(next) = &data.next_node_id {
                    self.current_node_id = next.clone();
                    return true; // Auto-advance
                }
            }
            StoryNodeVariant::Dialogue(data) => {
                let speaker = if data.speaker_id.is_empty() { "Narrator" } else { &data.speaker_id };
                let text = Self::get_text(&data.text);
                println!("\n[{}] {}", speaker, text);
                
                if let Some(next) = &data.next_node_id {
                    self.current_node_id = next.clone();
                    self.wait_for_enter();
                    return true;
                } else {
                    // Dead end
                    self.running = false;
                }
            }
            StoryNodeVariant::Choice(data) => {
                let prompt = Self::get_text(&data.prompt);
                println!("\n? {}", prompt);
                
                for (i, opt) in data.options.iter().enumerate() {
                    let opt_text = Self::get_text(&opt.text);
                    println!("  {}. {}", i + 1, opt_text);
                }

                let choice = if self.auto_mode {
                    let mut rng = rand::thread_rng();
                    let c = (rand::Rng::gen::<usize>(&mut rng) % data.options.len()) + 1;
                    println!("> [AUTO] Selecting: {}", c);
                    c
                } else {
                    loop {
                        print!("> ");
                        io::stdout().flush().unwrap();
                        
                        let mut input = String::new();
                        io::stdin().read_line(&mut input).unwrap();
                        
                        if let Ok(choice) = input.trim().parse::<usize>() {
                            if choice >= 1 && choice <= data.options.len() {
                                break choice;
                            }
                        }
                        println!("Invalid choice. Enter 1-{}", data.options.len());
                    }
                };

                let target = &data.options[choice - 1].target_node_id;
                self.current_node_id = target.clone();
                return true;
            }
            StoryNodeVariant::SetFlag(data) => {
                println!("\n[SET] {} = {:?}", data.flag, data.value);
                self.flags.insert(data.flag.clone(), data.value.clone());
                if let Some(next) = &data.next_node_id {
                    self.current_node_id = next.clone();
                    return true;
                }
            }
            StoryNodeVariant::Branch(data) => {
                let result = if let Some(flag) = &data.condition_flag {
                    self.flags.get(flag).and_then(|v| v.as_bool()).unwrap_or(false)
                } else {
                    false
                };
                println!("\n[BRANCH] {} -> {}", data.condition_flag.as_deref().unwrap_or("none"), result);
                let target = if result { &data.true_node_id } else { &data.false_node_id };
                if let Some(t) = target {
                    self.current_node_id = t.clone();
                    return true;
                }
            }
            StoryNodeVariant::Battle(data) => {
                println!("\n[BATTLE] Start vs {}", data.enemy_id);
                let won = if self.auto_mode {
                    rand::random::<bool>()
                } else {
                    println!("1. Win Battle\n2. Lose Battle");
                    loop {
                        print!("> ");
                        io::stdout().flush().unwrap();
                        let mut input = String::new();
                        io::stdin().read_line(&mut input).unwrap();
                        match input.trim().as_ref() {
                            "1" => break true,
                            "2" => break false,
                            _ => println!("Invalid. Enter 1 or 2."),
                        }
                    }
                };
                println!("[BATTLE] Result: {}", if won { "Victory" } else { "Defeat" });
                let target = if won { &data.win_node_id } else { &data.loss_node_id };
                if let Some(t) = target.as_ref() {
                    self.current_node_id = t.clone();
                    return true;
                }
            }
            StoryNodeVariant::SubGraph(data) => {
                println!("\n[SUBGRAPH] Entering: {}", data.graph_id);
                // In CLI, we just advance to next if possible
                if let Some(next) = &data.next_node_id {
                    self.current_node_id = next.clone();
                    return true;
                }
            }
            StoryNodeVariant::End(_) => {
                println!("\n=== THE END ===");
                self.running = false;
            }
            StoryNodeVariant::Unknown => {
                println!("\n[WARN] Unsupported node type: {}", node.node_type);
                self.running = false;
            }
        }
        false
    }

    fn wait_for_enter(&self) {
        if self.auto_mode { return; }
        print!("(Press Enter to continue) ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
    }

    fn run(&mut self) {
        while self.running {
            if !self.process_current_node() {
                break;
            }
        }
        println!("\nThank you for playing!");
    }
}

// ============================================================================
// Main
// ============================================================================

fn main() {
    println!("===========================================");
    println!("  DJ Engine - CLI Text RPG Runner v0.1   ");
    println!("===========================================\n");

    let args: Vec<String> = std::env::args().collect();
    
    let mut auto_mode = false;
    let mut graph_path = "games/cli_test_game/story_graphs/test_game.json".to_string();

    for arg in args.iter().skip(1) {
        if arg == "--auto" {
            auto_mode = true;
        } else {
            graph_path = arg.clone();
        }
    }

    println!("Loading story from: {}", graph_path);
    
    let path = Path::new(&graph_path);
    if !path.exists() {
        eprintln!("ERROR: File not found: {}", graph_path);
        eprintln!("\nUsage: cli_rpg <path_to_story_graph.json>");
        std::process::exit(1);
    }

    let content = match std::fs::read_to_string(path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("ERROR: Failed to read file: {}", e);
            std::process::exit(1);
        }
    };

    let graph: StoryGraphData = match serde_json::from_str(&content) {
        Ok(g) => g,
        Err(e) => {
            eprintln!("ERROR: Failed to parse JSON: {}", e);
            eprintln!("\nMake sure the file is a valid StoryGraphData JSON.");
            std::process::exit(1);
        }
    };

    println!("Loaded graph: {} ({})", graph.name, graph.id);
    println!("Nodes: {}", graph.nodes.len());
    println!("\n-------------------------------------------\n");

    let mut state = GameState::new(graph, auto_mode);
    state.run();
}
