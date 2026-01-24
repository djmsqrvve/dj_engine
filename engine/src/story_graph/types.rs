use bevy::prelude::*;
use std::collections::HashMap;
use crate::audio::AudioCommand;
use crate::data::story::{StoryGraphData, StoryNodeVariant};

/// Unique identifier for a node in the graph.
pub type NodeId = usize;

/// Represents a single logic or content step in the story.
#[derive(Debug, Clone, Reflect)]
pub enum StoryNode {
    /// Show dialogue and wait for user confirmation.
    Dialogue {
        speaker: String,
        text: String,
        portrait: Option<String>,
        next: Option<NodeId>,
    },
    /// Present a set of choices to the player.
    Choice {
        speaker: String,
        prompt: String,
        options: Vec<GraphChoice>,
    },
    /// Play a sound effect or music track.
    Audio {
        command: AudioCommand,
        next: Option<NodeId>,
    },
    /// Change the background scene.
    Background {
        path: String,
        duration: f32,
        next: Option<NodeId>,
    },
    /// Execute a sub-graph (Scene Container).
    SubGraph {
        graph_id: String,
        next: Option<NodeId>,
    },
    /// Conditional branch based on a story flag.
    Branch {
        flag: String,
        if_true: Option<NodeId>,
        if_false: Option<NodeId>,
    },
    /// Set or unset a story flag.
    SetFlag {
        flag: String,
        value: bool,
        next: Option<NodeId>,
    },
    /// Wait for a specified duration in seconds.
    Wait {
        duration: f32,
        next: Option<NodeId>,
    },
    /// A generic event trigger for game-specific logic.
    Event {
        event_id: String,
        payload: String,
        next: Option<NodeId>,
    },
    /// Start execution of the graph.
    Start {
        next: Option<NodeId>,
    },
    /// End execution of the current graph.
    End,
}

/// A choice option within a Choice node.
#[derive(Debug, Clone, Reflect)]
pub struct GraphChoice {
    pub text: String,
    pub next: Option<NodeId>,
    pub flag_required: Option<String>,
}

/// The graph container holding all nodes.
#[derive(Resource, Default, Clone, Reflect)]
#[reflect(Resource)]
pub struct StoryGraph {
    pub nodes: HashMap<NodeId, StoryNode>,
    pub start_node: Option<NodeId>,
    pub next_id: usize,
}

impl StoryGraph {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            start_node: None,
            next_id: 0,
        }
    }

    pub fn add(&mut self, node: StoryNode) -> NodeId {
        let id = self.next_id;
        self.nodes.insert(id, node);
        self.next_id += 1;
        id
    }

    pub fn set_start(&mut self, id: NodeId) {
        self.start_node = Some(id);
    }
}

/// Generic container for story flags (booleans).
#[derive(Resource, Default, Debug, Clone, Reflect)]
#[reflect(Resource)]
pub struct StoryFlags(pub HashMap<String, bool>);

impl StoryFlags {
    pub fn set(&mut self, flag: &str, value: bool) {
        self.0.insert(flag.to_string(), value);
    }

    pub fn get(&self, flag: &str) -> bool {
        *self.0.get(flag).unwrap_or(&false)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
pub enum ExecutionStatus {
    #[default]
    Idle,
    Running,
    WaitingForInput,
    WaitingForTimer,
    Paused,
}

#[derive(Resource, Default, Reflect)]
#[reflect(Resource)]
pub struct GraphExecutor {
    // For prototype simplicity, we store the struct directly.
    pub active_graph: Option<StoryGraph>,     
    pub current_node: Option<NodeId>,
    pub status: ExecutionStatus,
    pub wait_timer: Timer,
    /// Stack for sub-graph execution: (ParentGraph, ReturnNodeId)
    #[reflect(ignore)]
    pub stack: Vec<(StoryGraph, Option<NodeId>)>,
}

/// Library of loaded story graphs for sub-graph lookups.
#[derive(Resource, Default)]
pub struct StoryGraphLibrary {
    pub graphs: HashMap<String, StoryGraph>,
}

impl GraphExecutor {
    pub fn start(&mut self, graph: StoryGraph) {
        let start = graph.start_node;
        self.active_graph = Some(graph);
        self.current_node = start;
        self.status = ExecutionStatus::Running;
        self.stack.clear();
    }

    /// Helper to bridge Editor Data -> Runtime Graph
    pub fn load_from_data(&mut self, data: &StoryGraphData) {
        let mut graph = StoryGraph::new();
        let mut id_map: HashMap<String, NodeId> = HashMap::new();

        // Pass 1: Allocate IDs
        for node_data in &data.nodes {
            let next_id = graph.next_id; // Peek next ID
            // We insert a placeholder to reserve the ID
            graph.add(StoryNode::End); 
            id_map.insert(node_data.id.clone(), next_id);
        }

        // Pass 2: Overwrite with actual data
        for node_data in &data.nodes {
            let runtime_id = id_map[&node_data.id];
            
            let resolve = |opt_id: &Option<String>| -> Option<NodeId> {
                opt_id.as_ref().and_then(|id| id_map.get(id).cloned())
            };
            
            let node = match &node_data.data {
                StoryNodeVariant::Start(d) => StoryNode::Start {
                    next: resolve(&d.next_node_id),
                },
                StoryNodeVariant::Dialogue(d) => StoryNode::Dialogue {
                    speaker: d.speaker_id.clone(),
                    text: d.text.get("en").cloned().unwrap_or_default(),
                    portrait: d.portrait_id.clone(),
                    next: resolve(&d.next_node_id),
                },
                StoryNodeVariant::Choice(c) => StoryNode::Choice {
                    speaker: "Player".into(), // Default?
                    prompt: c.prompt.get("en").cloned().unwrap_or_default(),
                    options: c.options.iter().map(|o| GraphChoice {
                        text: o.text.get("en").cloned().unwrap_or_default(),
                        next: Some(id_map[&o.target_node_id]), // Choices must have targets?
                        flag_required: None,
                    }).collect(),
                },
                StoryNodeVariant::Action(a) => {
                     StoryNode::Event {
                         event_id: "lua_script".into(),
                         payload: a.lua_script_id.clone(),
                         next: resolve(&a.next_node_id),
                     }
                },
                StoryNodeVariant::End(e) => {
                    if let Some(scene) = &e.target_scene_id {
                        StoryNode::Background {
                            path: scene.clone(),
                            duration: 1.0,
                            next: None, 
                        }
                    } else {
                        StoryNode::End
                    }
                },
                StoryNodeVariant::SubGraph(s) => {
                    StoryNode::SubGraph {
                        graph_id: s.graph_id.clone(),
                        next: resolve(&s.next_node_id),
                    }
                },
                _ => StoryNode::End, // Unimplemented variants
            };

            graph.nodes.insert(runtime_id, node);
        }

        if let Some(start_id) = id_map.get(&data.root_node_id) {
            graph.set_start(*start_id);
        }

        self.start(graph);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default, Reflect)]
pub enum NodeAction {
    #[default]
    Advance,
    WaitInput,
    WaitTimer(f32),
    Jump(NodeId),
    End,
}
