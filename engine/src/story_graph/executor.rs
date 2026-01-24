use super::events::*;
use super::types::*;
use crate::audio::AudioCommand;
use crate::scene::ChangeSceneEvent;
use bevy::prelude::*;

/// Event sent to scripting layer to trigger an action
#[derive(Event, Debug, Clone)]
pub struct StoryActionEvent {
    pub script_id: String,
    pub params: serde_json::Value,
}

pub fn execute_graph(
    executor_res: ResMut<GraphExecutor>,
    library: Option<Res<StoryGraphLibrary>>,
    mut flags: ResMut<StoryFlags>,
    mut audio_events: EventWriter<AudioCommand>,
    mut scene_events: EventWriter<ChangeSceneEvent>,
    mut flow_events: EventWriter<StoryFlowEvent>,
    mut action_events: EventWriter<StoryActionEvent>,
    mut input_events: EventReader<StoryInputEvent>,
    time: Res<Time>,
) {
    let executor = executor_res.into_inner();

    // 1. Handle Input (if waiting)
    if executor.status == ExecutionStatus::WaitingForInput {
        for event in input_events.read() {
            match event {
                StoryInputEvent::Advance => {
                    executor.status = ExecutionStatus::Running;
                }
                StoryInputEvent::SelectChoice(index) => {
                    handle_choice_selection(&mut *executor, *index);
                }
            }
        }
    }

    // 2. Handle Timer (if waiting)
    if executor.status == ExecutionStatus::WaitingForTimer {
        executor.wait_timer.tick(time.delta());
        if executor.wait_timer.finished() {
            executor.status = ExecutionStatus::Running;
            advance_node(&mut *executor);
        }
    }

    // 3. Process Execution Loop
    let mut loops = 0;
    while executor.status == ExecutionStatus::Running && loops < 100 {
        loops += 1;

        // Clone node to release borrow on executor
        let node_to_process = if let Some(graph) = &executor.active_graph {
            if let Some(node_id) = executor.current_node {
                graph.nodes.get(&node_id).cloned()
            } else {
                None
            }
        } else {
            None
        };

        if let Some(node) = node_to_process {
            let action = process_node(
                &node,
                &library,
                &mut executor.stack,
                &mut executor.active_graph,
                &mut flags,
                &mut flow_events,
                &mut audio_events,
                &mut scene_events,
                &mut action_events,
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
                    advance_node(&mut *executor);
                }
                NodeAction::Jump(target_id) => {
                    executor.current_node = Some(target_id);
                }
                NodeAction::End => {
                    // Check stack for return
                    if let Some((parent_graph, return_node)) = executor.stack.pop() {
                        info!("Returning from sub-graph to parent");
                        executor.active_graph = Some(parent_graph);
                        executor.current_node = return_node;
                    } else {
                        executor.status = ExecutionStatus::Idle;
                        flow_events.send(StoryFlowEvent::GraphComplete);
                    }
                }
            }
        } else {
            // End of graph reached (null next) or invalid node
            // Check stack
            if let Some((parent_graph, return_node)) = executor.stack.pop() {
                info!("Returning from sub-graph (implicit end)");
                executor.active_graph = Some(parent_graph);
                executor.current_node = return_node;
            } else {
                executor.status = ExecutionStatus::Idle;
                flow_events.send(StoryFlowEvent::GraphComplete);
            }
        }
    }
}

fn advance_node(executor: &mut GraphExecutor) {
    let next_id = if let Some(graph) = &executor.active_graph {
        if let Some(node_id) = executor.current_node {
            if let Some(node) = graph.nodes.get(&node_id) {
                match node {
                    StoryNode::Dialogue { next, .. } => *next,
                    StoryNode::Audio { next, .. } => *next,
                    StoryNode::Background { next, .. } => *next,
                    StoryNode::Wait { next, .. } => *next,
                    StoryNode::SetFlag { next, .. } => *next,
                    StoryNode::Event { next, .. } => *next,
                    StoryNode::Start { next, .. } => *next,
                    StoryNode::SubGraph { next, .. } => *next, // Fallback if not processed as jump
                    _ => None,
                }
            } else {
                None
            }
        } else {
            None
        }
    } else {
        None
    };

    executor.current_node = next_id;
}

fn handle_choice_selection(executor: &mut GraphExecutor, index: usize) {
    let next_id = if let Some(graph) = &executor.active_graph {
        if let Some(node_id) = executor.current_node {
            if let Some(StoryNode::Choice { options, .. }) = graph.nodes.get(&node_id) {
                options.get(index).and_then(|opt| opt.next)
            } else {
                None
            }
        } else {
            None
        }
    } else {
        None
    };

    executor.current_node = next_id;
    executor.status = ExecutionStatus::Running;
}

fn process_node(
    node: &StoryNode,
    library: &Option<Res<StoryGraphLibrary>>,
    stack: &mut Vec<(StoryGraph, Option<NodeId>)>,
    active_graph: &mut Option<StoryGraph>,
    flags: &mut StoryFlags,
    flow: &mut EventWriter<StoryFlowEvent>,
    audio: &mut EventWriter<AudioCommand>,
    scene: &mut EventWriter<ChangeSceneEvent>,
    action_events: &mut EventWriter<StoryActionEvent>,
) -> NodeAction {
    match node {
        StoryNode::Dialogue {
            speaker,
            text,
            portrait,
            ..
        } => {
            flow.send(StoryFlowEvent::ShowDialogue {
                speaker: speaker.clone(),
                text: text.clone(),
                portrait: portrait.clone(),
            });
            NodeAction::WaitInput
        }
        StoryNode::Choice {
            prompt, options, ..
        } => {
            let option_texts = options.iter().map(|o| o.text.clone()).collect();
            flow.send(StoryFlowEvent::ShowChoices {
                prompt: prompt.clone(),
                options: option_texts,
            });
            NodeAction::WaitInput
        }
        StoryNode::Audio { command, .. } => {
            audio.send(command.clone());
            NodeAction::Advance
        }
        StoryNode::Background { path, duration, .. } => {
            scene.send(ChangeSceneEvent {
                background_path: path.clone(),
                duration: *duration,
            });
            NodeAction::Advance
        }
        StoryNode::Wait { duration, .. } => NodeAction::WaitTimer(*duration),
        StoryNode::Branch {
            flag,
            if_true,
            if_false,
        } => {
            if flags.get(flag) {
                if let Some(id) = if_true {
                    NodeAction::Jump(*id)
                } else {
                    NodeAction::Advance
                }
            } else if let Some(id) = if_false {
                NodeAction::Jump(*id)
            } else {
                NodeAction::Advance
            }
        }
        StoryNode::SetFlag { flag, value, .. } => {
            flags.set(flag, *value);
            NodeAction::Advance
        }
        StoryNode::SubGraph { graph_id, next } => {
            // Logic:
            // 1. Check if graph exists in library
            // 2. Clone current active_graph (parent)
            // 3. Push (parent, next) to stack
            // 4. Set active_graph = new graph
            // 5. Set current_node = new graph.start_node
            // 6. Return Jump(start_node)

            if let Some(lib) = library {
                if let Some(sub_graph) = lib.graphs.get(graph_id) {
                    if let Some(parent) = active_graph.take() {
                        stack.push((parent, *next));
                        *active_graph = Some(sub_graph.clone());
                        if let Some(start) = sub_graph.start_node {
                            return NodeAction::Jump(start);
                        } else {
                            warn!("SubGraph {} has no start node!", graph_id);
                            // Restore
                            if let Some((parent, _)) = stack.pop() {
                                *active_graph = Some(parent);
                            }
                            return NodeAction::Advance;
                        }
                    }
                } else {
                    error!("SubGraph ID not found in library: {}", graph_id);
                }
            } else {
                error!("StoryGraphLibrary resource missing!");
            }
            NodeAction::Advance
        }
        StoryNode::Event {
            event_id, payload, ..
        } => {
            // Bridge to StoryAction
            action_events.send(StoryActionEvent {
                script_id: event_id.clone(),
                params: serde_json::Value::String(payload.clone()),
            });
            NodeAction::Advance
        }
        StoryNode::End => NodeAction::End,
        StoryNode::Start { .. } => NodeAction::Advance,
    }
}
