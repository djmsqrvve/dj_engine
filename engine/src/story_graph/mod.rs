//! Node-based story execution system.
//!
//! Replaces linear dialogue queues with a directed graph of nodes.
//! Supports branching logic, events, and complex narrative flow.

use bevy::prelude::*;
use std::collections::HashMap;
use crate::audio::AudioCommand;
use crate::scene::ChangeSceneEvent;

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
    Scene {
        path: String,
        duration: f32,
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
    next_id: usize,
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
}

impl GraphExecutor {
    pub fn start(&mut self, graph: StoryGraph) {
        let start = graph.start_node;
        self.active_graph = Some(graph);
        self.current_node = start;
        self.status = ExecutionStatus::Running;
    }
}

/// Events sent FROM the Executor TO the UI/Game
#[derive(Event, Debug, Clone)]
pub enum StoryFlowEvent {
    ShowDialogue { speaker: String, text: String, portrait: Option<String> },
    ShowChoices { prompt: String, options: Vec<String> }, // Only send text to UI
    GraphComplete,
}

/// Events sent FROM the UI/Game TO the Executor
#[derive(Event, Debug, Clone)]
pub enum StoryInputEvent {
    Advance,
    SelectChoice(usize),
}

#[derive(Event)]
pub struct StoryEvent {
    pub id: String,
    pub payload: String,
}

pub struct StoryGraphPlugin;

impl Plugin for StoryGraphPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<StoryGraph>()
           .register_type::<StoryNode>()
           .register_type::<GraphChoice>()
           .register_type::<StoryFlags>()
           .register_type::<ExecutionStatus>()
           .register_type::<GraphExecutor>()
           .init_resource::<GraphExecutor>()
           .init_resource::<StoryFlags>()
           .add_event::<StoryEvent>()
           .add_event::<StoryFlowEvent>()
           .add_event::<StoryInputEvent>()
           .add_systems(Update, execute_graph);
    }
}

enum NodeAction {
    WaitInput,
    WaitTimer(f32),
    Advance,
    Jump(NodeId),
    End,
}

fn execute_graph(
    mut executor: ResMut<GraphExecutor>,
    mut flags: ResMut<StoryFlags>,
    mut audio_events: EventWriter<AudioCommand>,
    mut scene_events: EventWriter<ChangeSceneEvent>,
    mut flow_events: EventWriter<StoryFlowEvent>,
    mut story_events: EventWriter<StoryEvent>,
    mut input_events: EventReader<StoryInputEvent>,
    time: Res<Time>,
) {
    // 1. Handle Input (if waiting)
    if executor.status == ExecutionStatus::WaitingForInput {
        for event in input_events.read() {
            match event {
                StoryInputEvent::Advance => {
                    executor.status = ExecutionStatus::Running;
                }
                StoryInputEvent::SelectChoice(index) => {
                    handle_choice_selection(&mut executor, *index);
                    // handle_choice_selection sets status to Running
                }
            }
        }
    }

    // 2. Handle Timer (if waiting)
    if executor.status == ExecutionStatus::WaitingForTimer {
        executor.wait_timer.tick(time.delta());
        if executor.wait_timer.finished() {
            executor.status = ExecutionStatus::Running;
            advance_node(&mut executor);
        }
    }

    // 3. Process Execution Loop
    // We loop to handle immediate transitions (Audio -> Scene -> Branch -> Dialogue) in one frame
    let mut loops = 0;
    while executor.status == ExecutionStatus::Running && loops < 100 {
        loops += 1;

        // In a real asset system we'd use Handle and Assets<StoryGraph>
        
        if let Some(graph) = &executor.active_graph {
            if let Some(node_id) = executor.current_node {
                if let Some(node) = graph.nodes.get(&node_id) {
                    let action = process_node(
                        node,
                        &mut flags,
                        &mut flow_events,
                        &mut audio_events,
                        &mut scene_events,
                        &mut story_events
                    );

                    match action {
                        NodeAction::WaitInput => {
                            executor.status = ExecutionStatus::WaitingForInput;
                        }
                        NodeAction::WaitTimer(duration) => {
                            executor.status = ExecutionStatus::WaitingForTimer;
                            executor.wait_timer = Timer::from_seconds(duration, TimerMode::Once);
                        }
                        NodeAction::Advance => {
                            advance_node(&mut executor);
                        }
                        NodeAction::Jump(target_id) => {
                            executor.current_node = Some(target_id);
                        }
                        NodeAction::End => {
                            executor.status = ExecutionStatus::Idle;
                            flow_events.send(StoryFlowEvent::GraphComplete);
                        }
                    }
                } else {
                    executor.status = ExecutionStatus::Idle;
                }
            } else {
                executor.status = ExecutionStatus::Idle;
                flow_events.send(StoryFlowEvent::GraphComplete);
            }
        } else {
            executor.status = ExecutionStatus::Idle;
        }
    }
}

fn advance_node(executor: &mut GraphExecutor) {
    // Helper to move to the 'default next' of the current node
    // This duplicates logic inside process_node if we aren't careful, 
    // but process_node returns 'Advance' meaning "Go to my .next field".
    
    // We need to peek at the current node to know its next.
    // This is slightly inefficient but safe.
    let next_id = if let Some(graph) = &executor.active_graph {
        if let Some(node_id) = executor.current_node {
            if let Some(node) = graph.nodes.get(&node_id) {
                 match node {
                    StoryNode::Dialogue { next, .. } => *next,
                    StoryNode::Audio { next, .. } => *next,
                    StoryNode::Scene { next, .. } => *next,
                    StoryNode::Wait { next, .. } => *next,
                    StoryNode::SetFlag { next, .. } => *next,
                    StoryNode::Event { next, .. } => *next,
                    _ => None,
                }
            } else { None }
        } else { None }
    } else { None };
    
    executor.current_node = next_id;
}

fn handle_choice_selection(executor: &mut GraphExecutor, index: usize) {
    let next_id = if let Some(graph) = &executor.active_graph {
        if let Some(node_id) = executor.current_node {
             if let StoryNode::Choice { options, .. } = &graph.nodes[&node_id] {
                 options.get(index).and_then(|opt| opt.next)
             } else { None }
        } else { None }
    } else { None };

    executor.current_node = next_id;
    executor.status = ExecutionStatus::Running;
}

fn process_node(
    node: &StoryNode,
    flags: &mut StoryFlags,
    flow: &mut EventWriter<StoryFlowEvent>,
    audio: &mut EventWriter<AudioCommand>,
    scene: &mut EventWriter<ChangeSceneEvent>,
    story: &mut EventWriter<StoryEvent>,
) -> NodeAction {
    match node {
        StoryNode::Dialogue { speaker, text, portrait, .. } => {
            flow.send(StoryFlowEvent::ShowDialogue { 
                speaker: speaker.clone(), 
                text: text.clone(), 
                portrait: portrait.clone() 
            });
            NodeAction::WaitInput
        }
        StoryNode::Choice { prompt, options, .. } => {
            let option_texts = options.iter().map(|o| o.text.clone()).collect();
            flow.send(StoryFlowEvent::ShowChoices { 
                prompt: prompt.clone(), 
                options: option_texts 
            });
            NodeAction::WaitInput
        }
        StoryNode::Audio { command, .. } => {
            audio.send(command.clone());
            NodeAction::Advance
        }
        StoryNode::Scene { path, duration, .. } => {
            scene.send(ChangeSceneEvent { 
                background_path: path.clone(), 
                duration: *duration 
            });
            NodeAction::Advance // Or WaitTimer if we want to block? For now Advance.
        }
        StoryNode::Wait { duration, .. } => {
            NodeAction::WaitTimer(*duration)
        }
        StoryNode::Branch { flag, if_true, if_false } => {
            if flags.get(flag) {
                if let Some(id) = if_true { NodeAction::Jump(*id) } else { NodeAction::Advance }
            } else {
                if let Some(id) = if_false { NodeAction::Jump(*id) } else { NodeAction::Advance }
            }
        }
        StoryNode::SetFlag { flag, value, .. } => {
            flags.set(flag, *value);
            NodeAction::Advance
        }
        StoryNode::Event { event_id, payload, .. } => {
            story.send(StoryEvent { id: event_id.clone(), payload: payload.clone() });
            NodeAction::Advance
        }
        StoryNode::End => {
            NodeAction::End
        }
    }
}

