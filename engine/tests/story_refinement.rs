use bevy::prelude::*;
use dj_engine::prelude::*;
use dj_engine::story_graph::types::{
    ExecutionStatus, FlagValue, GraphExecutor, StoryFlags, StoryGraph, StoryGraphLibrary, StoryNode, StoryCondition,
};
use dj_engine::data::story::{StoryGraphData, StoryNodeData, StoryNodeVariant, StartNodeData, SetFlagNodeData, FlagValue as DataFlagValue};

#[test]
fn test_iteration_limit() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app.add_plugins(AssetPlugin::default());
    app.add_plugins(bevy::state::app::StatesPlugin);
    app.add_plugins(bevy::input::InputPlugin);
    app.init_asset::<AudioSource>();
    app.add_plugins(DJEnginePlugin::default().without_diagnostics());

    // Create an infinite loop: Start -> Start
    let mut graph = StoryGraph::new("loop_test");
    let start = graph.add(StoryNode::Start { next: Some(0) });
    graph.set_start(start);

    let id = graph.id.clone();
    let start_node = graph.start_node;
    app.world_mut().resource_mut::<StoryGraphLibrary>().graphs.insert(id.clone(), graph);

    let mut executor = app.world_mut().resource_mut::<GraphExecutor>();
    executor.start(id, start_node);

    // One update should trigger the 100 iteration limit and pause
    app.update();

    let executor = app.world().resource::<GraphExecutor>();
    assert_eq!(executor.status, ExecutionStatus::Paused, "Should have paused due to iteration limit");
}

#[test]
fn test_subgraph_recursion_limit() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app.add_plugins(AssetPlugin::default());
    app.add_plugins(bevy::state::app::StatesPlugin);
    app.add_plugins(bevy::input::InputPlugin);
    app.init_asset::<AudioSource>();
    app.add_plugins(DJEnginePlugin::default().without_diagnostics());

    // Create a graph that calls itself
    let mut graph = StoryGraph::new("recurse");
    let start = graph.add(StoryNode::SubGraph {
        graph_id: "recurse".into(),
        next: None,
    });
    graph.set_start(start);

    let mut library = app.world_mut().resource_mut::<StoryGraphLibrary>();
    library.graphs.insert("recurse".into(), graph.clone());

    let mut executor = app.world_mut().resource_mut::<GraphExecutor>();
    executor.start("recurse".into(), graph.start_node);

    // Run until it hits the limit
    // With recursion cycle detection, it should actually stop immediately or loop?
    // My implementation checks active_graph_id matches.
    // If graph calls itself, active_graph_id is "recurse".
    // SubGraph "recurse" called.
    // Cycle check: `active_graph_id` == "recurse". Detected!
    // Returns `Advance`.
    // Next is None. End.
    // So recursion depth shouldn't increase indefinitely with cycle detection.
    // It should handle it gracefully.
    
    app.update();

    let executor = app.world().resource::<GraphExecutor>();
    // Depth should be 0 because it refused to recurse.
    assert_eq!(executor.current_depth, 0);
}

#[test]
fn test_typed_flags() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app.add_plugins(AssetPlugin::default());
    app.add_plugins(bevy::state::app::StatesPlugin);
    app.add_plugins(bevy::input::InputPlugin);
    app.init_asset::<AudioSource>();
    app.add_plugins(DJEnginePlugin::default().without_diagnostics());

    let mut graph = StoryGraph::new("typed_flags");
    
    // 0: Set score = 100
    let s0 = graph.add(StoryNode::SetFlag {
        flag: "score".into(),
        value: FlagValue::Number(100.0),
        next: Some(1),
    });
    
    // 1: Branch if score > 50
    let _s1 = graph.add(StoryNode::Branch {
        condition: StoryCondition::GreaterThan("score".into(), 50.0),
        if_true: Some(2),
        if_false: Some(3),
    });
    
    // 2: Set result = "win"
    let _s2 = graph.add(StoryNode::SetFlag {
        flag: "result".into(),
        value: FlagValue::String("win".into()),
        next: Some(4),
    });
    
    // 3: Set result = "lose"
    let _s3 = graph.add(StoryNode::SetFlag {
        flag: "result".into(),
        value: FlagValue::String("lose".into()),
        next: Some(4),
    });
    
    // 4: End
    let _s4 = graph.add(StoryNode::End);
    
    graph.set_start(s0);

    let id = graph.id.clone();
    let start = graph.start_node;
    app.world_mut().resource_mut::<StoryGraphLibrary>().graphs.insert(id.clone(), graph);

    let mut executor = app.world_mut().resource_mut::<GraphExecutor>();
    executor.start(id, start);

    app.update(); // Set score, evaluates branch, sets result

    let flags = app.world().resource::<StoryFlags>();
    match flags.get("result") {
        Some(FlagValue::String(s)) => assert_eq!(s, "win"),
        _ => panic!("Expected string result 'win'"),
    }
}

#[test]
fn test_graph_validation() {
    // 1. Cycle detection
    let mut loop_graph = StoryGraphData::new("loop", "Looping Graph");
    loop_graph.nodes.push(StoryNodeData {
        id: "A".into(),
        data: StoryNodeVariant::Start(StartNodeData { next_node_id: Some("B".into()) }),
        ..Default::default()
    });
    loop_graph.nodes.push(StoryNodeData {
        id: "B".into(),
        data: StoryNodeVariant::SetFlag(SetFlagNodeData { 
            flag: "x".into(), 
            value: DataFlagValue::Bool(true), 
            next_node_id: Some("A".into()) 
        }),
        ..Default::default()
    });
    loop_graph.root_node_id = "A".into();

    let errors = loop_graph.validate();
    assert!(errors.iter().any(|e| matches!(e, dj_engine::data::story::ValidationError::CycleDetected(_))));

    // 2. Orphan detection
    let mut orphan_graph = StoryGraphData::new("orphan", "Orphaned Node");
    orphan_graph.nodes.push(StoryNodeData {
        id: "Start".into(),
        data: StoryNodeVariant::Start(StartNodeData { next_node_id: None }),
        ..Default::default()
    });
    orphan_graph.nodes.push(StoryNodeData {
        id: "Lonely".into(),
        data: StoryNodeVariant::Dialogue(Default::default()),
        ..Default::default()
    });
    orphan_graph.root_node_id = "Start".into();

    let errors = orphan_graph.validate();
    assert!(errors.iter().any(|e| matches!(e, dj_engine::data::story::ValidationError::UnreachableNode(id) if id == "Lonely")));
}