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
    lua: Option<Res<crate::scripting::LuaContext>>,
    mut flags: ResMut<StoryFlags>,
    mut inventory: Option<ResMut<crate::game::Inventory>>,
    mut quest_log: Option<ResMut<crate::game::QuestLog>>,
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
                    handle_choice_selection(&mut *executor, *index, &library, &mut flags, inventory.as_deref_mut(), quest_log.as_deref_mut());
                }
            }
        }
    }

    // 2. Handle Timer (if waiting)
    if executor.status == ExecutionStatus::WaitingForTimer {
        executor.wait_timer.tick(time.delta());
        if executor.wait_timer.finished() {
            executor.status = ExecutionStatus::Running;
            advance_node(&mut *executor, &library, &mut flags, inventory.as_deref_mut(), quest_log.as_deref_mut());
        }
    }

    // 3. Process Execution Loop
    let mut loops = 0;
    while executor.status == ExecutionStatus::Running && loops < MAX_NODES_PER_FRAME {
        loops += 1;
        if loops >= MAX_NODES_PER_FRAME {
            warn!("StoryGraph execution budget exceeded ({})! Breaking potential infinite loop.", MAX_NODES_PER_FRAME);
            executor.status = ExecutionStatus::Paused; // Pause to avoid hang
            break;
        }

        // Get active graph ID
        let graph_id = executor.active_graph_id.clone();
        
        // Retrieve graph from library
        let graph_node = if let (Some(id), Some(lib)) = (&graph_id, &library) {
            if let Some(graph) = lib.graphs.get(id) {
                if let Some(node_id) = executor.current_node {
                    graph.nodes.get(&node_id).cloned()
                } else {
                    None
                }
            } else {
                error!("Active graph ID '{}' not found in library!", id);
                executor.status = ExecutionStatus::Idle;
                None
            }
        } else {
            None
        };

        if let Some(node) = graph_node {
            let action = process_node(
                &node,
                &library,
                &lua,
                &mut executor.stack,
                &mut executor.active_graph_id,
                &mut executor.current_depth,
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
                    advance_node(&mut *executor, &library, &mut flags, inventory.as_deref_mut(), quest_log.as_deref_mut());
                }
                NodeAction::Jump(target_id) => {
                    executor.current_node = Some(target_id);
                }
                NodeAction::End => {
                    // Check stack for return
                    if let Some((parent_graph_id, return_node)) = executor.stack.pop() {
                        info!("Returning from sub-graph to parent: {}", parent_graph_id);
                        executor.active_graph_id = Some(parent_graph_id);
                        executor.current_node = return_node;
                        executor.current_depth = executor.current_depth.saturating_sub(1);
                    } else {
                        executor.status = ExecutionStatus::Idle;
                        flow_events.send(StoryFlowEvent::GraphComplete);
                    }
                }
            }
        } else {
            // End of graph reached (null next) or invalid node
            // Check stack
            if let Some((parent_graph_id, return_node)) = executor.stack.pop() {
                info!("Returning from sub-graph (implicit end)");
                executor.active_graph_id = Some(parent_graph_id);
                executor.current_node = return_node;
                executor.current_depth = executor.current_depth.saturating_sub(1);
            } else {
                executor.status = ExecutionStatus::Idle;
                flow_events.send(StoryFlowEvent::GraphComplete);
            }
        }
    }
}

fn advance_node(
    executor: &mut GraphExecutor,
    library: &Option<Res<StoryGraphLibrary>>,
    flags: &mut StoryFlags,
    mut inventory: Option<&mut crate::game::Inventory>,
    mut quest_log: Option<&mut crate::game::QuestLog>,
) {
    let (next_id, effects) = if let (Some(id), Some(lib)) = (&executor.active_graph_id, library) {
        if let Some(graph) = lib.graphs.get(id) {
            if let Some(node_id) = executor.current_node {
                if let Some(node) = graph.nodes.get(&node_id) {
                    match node {
                        StoryNode::Dialogue { next, effects, .. } => (*next, Some(effects.clone())),
                        StoryNode::Audio { next, .. } => (*next, None),
                        StoryNode::Background { next, .. } => (*next, None),
                        StoryNode::Wait { next, .. } => (*next, None),
                        StoryNode::SetFlag { next, .. } => (*next, None),
                        StoryNode::Event { next, .. } => (*next, None),
                        StoryNode::Camera { next, .. } => (*next, None),
                        StoryNode::TimeControl { next, .. } => (*next, None),
                        StoryNode::Start { next, .. } => (*next, None),
                        StoryNode::SubGraph { next, .. } => (*next, None),
                        _ => (None, None),
                    }
                } else { (None, None) }
            } else { (None, None) }
        } else { (None, None) }
    } else { (None, None) };

    if let Some(effs) = effects {
        for effect in effs {
            apply_effect(&effect, flags, inventory.as_deref_mut(), quest_log.as_deref_mut());
        }
    }

    executor.current_node = next_id;
}

fn handle_choice_selection(
    executor: &mut GraphExecutor,
    index: usize,
    library: &Option<Res<StoryGraphLibrary>>,
    flags: &mut StoryFlags,
    mut inventory: Option<&mut crate::game::Inventory>,
    mut quest_log: Option<&mut crate::game::QuestLog>,
) {
    let (next_id, effects) = if let (Some(id), Some(lib)) = (&executor.active_graph_id, library) {
        if let Some(graph) = lib.graphs.get(id) {
            if let Some(node_id) = executor.current_node {
                if let Some(StoryNode::Choice { options, .. }) = graph.nodes.get(&node_id) {
                    options.get(index).map(|opt| (opt.next, Some(opt.effects.clone()))).unwrap_or((None, None))
                } else { (None, None) }
            } else { (None, None) }
        } else { (None, None) }
    } else { (None, None) };

    if let Some(effs) = effects {
        for effect in effs {
            apply_effect(&effect, flags, inventory.as_deref_mut(), quest_log.as_deref_mut());
        }
    }

    executor.current_node = next_id;
    executor.status = ExecutionStatus::Running;
}

fn process_node(
    node: &StoryNode,
    library: &Option<Res<StoryGraphLibrary>>,
    lua: &Option<Res<crate::scripting::LuaContext>>,
    stack: &mut Vec<(String, Option<NodeId>)>,
    active_graph_id: &mut Option<String>,
    current_depth: &mut usize,
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
            condition,
            if_true,
            if_false,
        } => execute_conditional(condition, *if_true, *if_false, flags, lua),
        StoryNode::SetFlag { flag, value, .. } => {
            flags.set(flag, value.clone());
            NodeAction::Advance
        }
        StoryNode::SubGraph { graph_id, next } => {
            if let Some(lib) = library {
                if let Some(sub_graph) = lib.graphs.get(graph_id) {
                    if *current_depth >= MAX_SUBGRAPH_DEPTH {
                        error!(
                            "MAX_SUBGRAPH_DEPTH ({}) exceeded! Aborting SubGraph: {}",
                            MAX_SUBGRAPH_DEPTH, graph_id
                        );
                        return NodeAction::Advance;
                    }

                    // Check for recursion cycle
                    let mut cycle_detected = false;
                    if let Some(current) = active_graph_id {
                        if current == graph_id {
                            cycle_detected = true;
                        }
                    }
                    if !cycle_detected {
                        for (parent_id, _) in stack.iter() {
                            if parent_id == graph_id {
                                cycle_detected = true;
                                break;
                            }
                        }
                    }

                    if cycle_detected {
                         error!("Recursion cycle detected! Graph '{}' is already in the execution stack.", graph_id);
                         return NodeAction::Advance;
                    }

                    if let Some(parent_id) = active_graph_id.take() {
                        stack.push((parent_id, *next));
                        *active_graph_id = Some(graph_id.clone());
                        *current_depth += 1;

                        if let Some(start) = sub_graph.start_node {
                            return NodeAction::Jump(start);
                        } else {
                            warn!("SubGraph {} has no start node!", graph_id);
                            // Restore
                            if let Some((parent_id, _)) = stack.pop() {
                                *active_graph_id = Some(parent_id);
                                *current_depth -= 1;
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
        StoryNode::Camera {
            preset_id,
            position,
            zoom,
            duration,
            ..
        } => execute_camera(
            preset_id.clone(),
            *position,
            *zoom,
            *duration,
            flow,
        ),
        StoryNode::TimeControl {
            time_scale, pause, ..
        } => execute_time_control(*time_scale, *pause, flow),
        StoryNode::End => NodeAction::End,
        StoryNode::Start { .. } => NodeAction::Advance,
        // Fallback for safety/future-proofing
        #[allow(unreachable_patterns)]
        _ => {
            error!("Unimplemented or unknown StoryNode type encountered!");
            NodeAction::Advance
        }
    }
}

fn execute_conditional(
    condition: &StoryCondition,
    if_true: Option<NodeId>,
    if_false: Option<NodeId>,
    flags: &StoryFlags,
    lua: &Option<Res<crate::scripting::LuaContext>>,
) -> NodeAction {
    let result = if let StoryCondition::LuaExpression(script) = condition {
        if let Some(lua_ctx) = lua {
            if let Ok(lua_guard) = lua_ctx.lua.lock() {
                lua_guard.load(script).eval::<bool>().unwrap_or_else(|e| {
                    error!("Lua condition error: {}", e);
                    false
                })
            } else {
                error!("Lua mutex poisoned in condition check");
                false
            }
        } else {
            error!("Lua context missing for condition check");
            false
        }
    } else {
        flags.evaluate(condition)
    };

    if result {
        if let Some(id) = if_true {
            NodeAction::Jump(id)
        } else {
            NodeAction::Advance
        }
    } else if let Some(id) = if_false {
        NodeAction::Jump(id)
    } else {
        NodeAction::Advance
    }
}

fn execute_camera(
    preset_id: Option<String>,
    position: Vec3,
    zoom: f32,
    duration: f32,
    flow: &mut EventWriter<StoryFlowEvent>,
) -> NodeAction {
    flow.send(StoryFlowEvent::CameraControl {
        preset_id,
        position,
        zoom,
        duration,
    });
    NodeAction::Advance
}

fn execute_time_control(
    time_scale: f32,
    pause: bool,
    flow: &mut EventWriter<StoryFlowEvent>,
) -> NodeAction {
    flow.send(StoryFlowEvent::TimeControl {
        time_scale,
        pause,
    });
    NodeAction::Advance
}

fn apply_effect(
    effect: &crate::data::story::StoryEffect,
    flags: &mut StoryFlags,
    inventory: Option<&mut crate::game::Inventory>,
    quest_log: Option<&mut crate::game::QuestLog>,
) {
    use crate::data::story::EffectType;
    match effect.effect_type {
        EffectType::SetVar => {
            if let (Some(flag), Some(val)) = (
                effect.params.get("flag").and_then(|v| v.as_str()),
                effect.params.get("value"),
            ) {
                if let Some(b) = val.as_bool() {
                    flags.set(flag, FlagValue::Bool(b));
                } else if let Some(n) = val.as_f64() {
                    flags.set(flag, FlagValue::Number(n as f32));
                } else if let Some(s) = val.as_str() {
                    flags.set(flag, FlagValue::String(s.to_string()));
                }
            }
        }
        EffectType::AddVar => {
            if let (Some(flag), Some(val)) = (
                effect.params.get("flag").and_then(|v| v.as_str()),
                effect.params.get("value").and_then(|v| v.as_f64()),
            ) {
                let current = match flags.get(flag) {
                    Some(FlagValue::Number(n)) => *n,
                    _ => 0.0,
                };
                flags.set(flag, FlagValue::Number(current + val as f32));
            }
        }
        EffectType::GiveItem => {
            if let Some(item_id) = effect.params.get("item_id").and_then(|v| v.as_str()) {
                let qty = effect.params.get("quantity").and_then(|v| v.as_u64()).unwrap_or(1) as u32;
                if let Some(inv) = inventory {
                    inv.add_item(item_id, qty);
                }
            }
        }
        EffectType::RemoveItem => {
            if let Some(item_id) = effect.params.get("item_id").and_then(|v| v.as_str()) {
                let qty = effect.params.get("quantity").and_then(|v| v.as_u64()).unwrap_or(1) as u32;
                if let Some(inv) = inventory {
                    inv.remove_item(item_id, qty);
                }
            }
        }
        EffectType::SetQuestState => {
            if let (Some(id), Some(state_str)) = (
                effect.params.get("quest_id").and_then(|v| v.as_str()),
                effect.params.get("state").and_then(|v| v.as_str()),
            ) {
                if let Some(ql) = quest_log {
                    let state = match state_str {
                        "active" => crate::game::QuestState::Active("".into()),
                        "ready" => crate::game::QuestState::ReadyToTurnIn,
                        "completed" => crate::game::QuestState::Completed,
                        _ => crate::game::QuestState::NotStarted,
                    };
                    ql.set_state(id, state);
                }
            }
        }
    }
}