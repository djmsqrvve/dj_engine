use crate::audio::AudioCommand;
use crate::data::story::{StoryGraphData, StoryNodeVariant};
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Unique identifier for a node in the graph.
pub type NodeId = usize;

/// Supported types for story flags.
#[derive(Debug, Clone, Reflect, PartialEq, Serialize, Deserialize)]
pub enum FlagValue {
    Bool(bool),
    Number(f32),
    String(String),
}

impl Default for FlagValue {
    fn default() -> Self {
        Self::Bool(false)
    }
}

/// Condition logic for branching.
#[derive(Debug, Clone, Reflect, PartialEq, Serialize, Deserialize)]
pub enum StoryCondition {
    /// Flag is true (for booleans)
    IsTrue(String),
    /// Flag equals a certain value
    Equals(String, FlagValue),
    /// Flag is greater than a value
    GreaterThan(String, f32),
    /// Flag is less than a value
    LessThan(String, f32),
    /// Lua script returning boolean
    LuaExpression(String),
}

/// Represents a single logic or content step in the story.
#[derive(Debug, Clone, Reflect)]
pub enum StoryNode {
    /// Show dialogue and wait for user confirmation.
    Dialogue {
        speaker: String,
        text: String,
        portrait: Option<String>,
        next: Option<NodeId>,
        effects: Vec<crate::data::story::StoryEffect>,
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
    /// Conditional branch based on story logic.
    Branch {
        condition: StoryCondition,
        if_true: Option<NodeId>,
        if_false: Option<NodeId>,
    },
    /// Set or update a story flag.
    SetFlag {
        flag: String,
        value: FlagValue,
        next: Option<NodeId>,
    },
    /// Wait for a specified duration in seconds.
    Wait { duration: f32, next: Option<NodeId> },
    /// A generic event trigger for game-specific logic.
    Event {
        event_id: String,
        payload: String,
        next: Option<NodeId>,
    },
    /// Move or transition the camera.
    Camera {
        preset_id: Option<String>,
        position: Vec3,
        zoom: f32,
        duration: f32,
        next: Option<NodeId>,
    },
    /// Control time scale or pause gameplay.
    TimeControl {
        time_scale: f32,
        pause: bool,
        next: Option<NodeId>,
    },
    /// Start execution of the graph.
    Start { next: Option<NodeId> },
    /// End execution of the current graph.
    End,
}

/// A choice option within a Choice node.
#[derive(Debug, Clone, Reflect)]
pub struct GraphChoice {
    pub text: String,
    pub next: Option<NodeId>,
    pub conditions: Vec<StoryCondition>,
    pub effects: Vec<crate::data::story::StoryEffect>,
}

/// The graph container holding all nodes.
#[derive(Resource, Default, Clone, Reflect)]
#[reflect(Resource)]
pub struct StoryGraph {
    pub id: String,
    pub nodes: HashMap<NodeId, StoryNode>,
    pub start_node: Option<NodeId>,
    pub next_id: usize,
}

impl StoryGraph {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
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

/// Generic container for story flags.
#[derive(Resource, Default, Debug, Clone, Reflect)]
#[reflect(Resource)]
pub struct StoryFlags(pub HashMap<String, FlagValue>);

impl StoryFlags {
    pub fn set(&mut self, flag: &str, value: FlagValue) {
        self.0.insert(flag.to_string(), value);
    }

    pub fn get(&self, flag: &str) -> Option<&FlagValue> {
        self.0.get(flag)
    }

    /// Returns true if the flag exists in the state.
    pub fn exists(&self, flag: &str) -> bool {
        self.0.contains_key(flag)
    }

    /// Returns the boolean value of a flag. 
    /// Returns `None` if the flag is missing or not a boolean.
    pub fn get_bool_strict(&self, flag: &str) -> Option<bool> {
        match self.0.get(flag) {
            Some(FlagValue::Bool(b)) => Some(*b),
            _ => None,
        }
    }

    /// Returns the boolean value of a flag, defaulting to false if missing or type mismatch.
    pub fn get_bool(&self, flag: &str) -> bool {
        match self.0.get(flag) {
            Some(FlagValue::Bool(b)) => *b,
            _ => false,
        }
    }

    pub fn evaluate(&self, condition: &StoryCondition) -> bool {
        match condition {
            StoryCondition::IsTrue(f) => self.get_bool(f),
            StoryCondition::Equals(f, val) => self.0.get(f) == Some(val),
            StoryCondition::GreaterThan(f, target) => match self.0.get(f) {
                Some(FlagValue::Number(n)) => n > target,
                _ => false,
            },
            StoryCondition::LessThan(f, target) => match self.0.get(f) {
                Some(FlagValue::Number(n)) => n < target,
                _ => false,
            },
            StoryCondition::LuaExpression(_) => {
                error!("LuaExpression cannot be evaluated by StoryFlags directly! Use GraphExecutor.");
                false
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Reflect, Serialize, Deserialize)]
pub enum ExecutionStatus {
    #[default]
    Idle,
    Running,
    WaitingForInput,
    WaitingForTimer,
    Paused,
}

#[derive(Resource, Default, Reflect, Serialize, Deserialize)]
#[reflect(Resource)]
pub struct GraphExecutor {
    pub active_graph_id: Option<String>,
    pub current_node: Option<NodeId>,
    pub status: ExecutionStatus,
    #[reflect(ignore)]
    #[serde(skip)] // Timer is transient
    pub wait_timer: Timer,
    /// Stack for sub-graph execution: (ParentGraphID, ReturnNodeId)
    pub stack: Vec<(String, Option<NodeId>)>,
    /// Current subgraph nesting depth
    pub current_depth: usize,
}

pub const MAX_SUBGRAPH_DEPTH: usize = 50;
pub const MAX_NODES_PER_FRAME: usize = 100;

/// Library of loaded story graphs for sub-graph lookups.
#[derive(Resource, Default)]
pub struct StoryGraphLibrary {
    pub graphs: HashMap<String, StoryGraph>,
}

impl GraphExecutor {
    pub fn start(&mut self, graph_id: String, start_node: Option<NodeId>) {
        self.active_graph_id = Some(graph_id);
        self.current_node = start_node;
        self.status = ExecutionStatus::Running;
        self.stack.clear();
        self.current_depth = 0;
    }

    /// Helper to bridge Editor Data -> Runtime Graph
    pub fn load_from_data(&mut self, data: &StoryGraphData, library: &mut StoryGraphLibrary) {
        let mut graph = StoryGraph::new(data.id.clone());
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
                    effects: d.effects.clone(),
                },
                StoryNodeVariant::Choice(c) => StoryNode::Choice {
                    speaker: "Player".into(), // Default?
                    prompt: c.prompt.get("en").cloned().unwrap_or_default(),
                    options: c
                        .options
                        .iter()
                        .map(|o| GraphChoice {
                            text: o.text.get("en").cloned().unwrap_or_default(),
                            next: Some(id_map[&o.target_node_id]), // Choices must have targets?
                            conditions: o.conditions.iter().map(bridge_condition).collect(),
                            effects: o.effects.clone(),
                        })
                        .collect(),
                },
                StoryNodeVariant::Action(a) => StoryNode::Event {
                    event_id: "lua_script".into(),
                    payload: a.lua_script_id.clone(),
                    next: resolve(&a.next_node_id),
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
                }
                StoryNodeVariant::Conditional(c) => StoryNode::Branch {
                    condition: match &c.condition {
                        crate::data::story::StoryCondition::IsTrue { flag } => {
                            StoryCondition::IsTrue(flag.clone())
                        }
                        crate::data::story::StoryCondition::Equals { flag, value } => {
                            StoryCondition::Equals(flag.clone(), bridge_flag_value(value))
                        }
                        crate::data::story::StoryCondition::GreaterThan { flag, value } => {
                            StoryCondition::GreaterThan(flag.clone(), *value)
                        }
                        crate::data::story::StoryCondition::LessThan { flag, value } => {
                            StoryCondition::LessThan(flag.clone(), *value)
                        }
                        crate::data::story::StoryCondition::LuaExpression { script } => {
                            StoryCondition::LuaExpression(script.clone())
                        }
                    },
                    if_true: id_map.get(&c.true_target_node_id).cloned(),
                    if_false: id_map.get(&c.false_target_node_id).cloned(),
                },
                StoryNodeVariant::SetFlag(s) => StoryNode::SetFlag {
                    flag: s.flag.clone(),
                    value: bridge_flag_value(&s.value),
                    next: resolve(&s.next_node_id),
                },
                StoryNodeVariant::Camera(c) => StoryNode::Camera {
                    preset_id: c.preset_id.clone(),
                    position: Vec3::new(c.position.x, c.position.y, c.position.z),
                    zoom: c.zoom,
                    duration: c.duration,
                    next: resolve(&c.next_node_id),
                },
                StoryNodeVariant::TimeControl(t) => StoryNode::TimeControl {
                    time_scale: t.time_scale,
                    pause: t.pause_gameplay,
                    next: resolve(&t.next_node_id),
                },
                StoryNodeVariant::SubGraph(s) => StoryNode::SubGraph {
                    graph_id: s.graph_id.clone(),
                    next: resolve(&s.next_node_id),
                },
            };

            graph.nodes.insert(runtime_id, node);
        }

        if let Some(start_id) = id_map.get(&data.root_node_id) {
            graph.set_start(*start_id);
        }

        let start_node = graph.start_node;
        let id = graph.id.clone();
        library.graphs.insert(id.clone(), graph);
        self.start(id, start_node);
    }
}

fn bridge_flag_value(val: &crate::data::story::FlagValue) -> FlagValue {
    match val {
        crate::data::story::FlagValue::Bool(b) => FlagValue::Bool(*b),
        crate::data::story::FlagValue::Number(n) => FlagValue::Number(*n),
        crate::data::story::FlagValue::String(s) => FlagValue::String(s.clone()),
    }
}

fn bridge_condition(cond: &crate::data::story::StoryCondition) -> StoryCondition {
    match cond {
        crate::data::story::StoryCondition::IsTrue { flag } => StoryCondition::IsTrue(flag.clone()),
        crate::data::story::StoryCondition::Equals { flag, value } => {
            StoryCondition::Equals(flag.clone(), bridge_flag_value(value))
        }
        crate::data::story::StoryCondition::GreaterThan { flag, value } => {
            StoryCondition::GreaterThan(flag.clone(), *value)
        }
        crate::data::story::StoryCondition::LessThan { flag, value } => {
            StoryCondition::LessThan(flag.clone(), *value)
        }
        crate::data::story::StoryCondition::LuaExpression { script } => {
            StoryCondition::LuaExpression(script.clone())
        }
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
