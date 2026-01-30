use bevy::prelude::*;
use dj_engine::prelude::*;
use dj_engine::story_graph::types::{
    ExecutionStatus, FlagValue, GraphExecutor, StoryFlags, StoryGraph, StoryGraphLibrary, StoryNode,
};

#[test]
fn test_subgraph_execution() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app.add_plugins(AssetPlugin::default());
    app.add_plugins(bevy::input::InputPlugin);
    app.add_plugins(bevy::state::app::StatesPlugin);
    app.init_asset::<AudioSource>();

    // Manually register StoryGraphPlugin resources/events if we don't want the full DJEngine
    // But DJEnginePlugin is cleaner.
    app.add_plugins(DJEnginePlugin::default().without_diagnostics());

    // 1. Setup Library with a Sub-Graph
    let mut sub_graph = StoryGraph::new("my_scene");
    // SubNode 0: Set flag 'in_subgraph'
    let s0 = sub_graph.add(StoryNode::SetFlag {
        flag: "in_subgraph".to_string(),
        value: FlagValue::Bool(true),
        next: Some(1),
    });
    // SubNode 1: End
    let _s1 = sub_graph.add(StoryNode::End);
    sub_graph.set_start(s0);

    let mut library = app.world_mut().resource_mut::<StoryGraphLibrary>();
    library.graphs.insert("my_scene".to_string(), sub_graph);

    // 2. Setup Main Graph
    let mut main_graph = StoryGraph::new("main");
    // Node 0: SubGraph call
    let m0 = main_graph.add(StoryNode::SubGraph {
        graph_id: "my_scene".to_string(),
        next: Some(1),
    });
    // Node 1: Set flag 'back_in_main'
    let _m1 = main_graph.add(StoryNode::SetFlag {
        flag: "back_in_main".to_string(),
        value: FlagValue::Bool(true),
        next: Some(2),
    });
    // Node 2: End
    let _m2 = main_graph.add(StoryNode::End);
    main_graph.set_start(m0);

    let main_id = main_graph.id.clone();
    let main_start = main_graph.start_node;
    app.world_mut().resource_mut::<StoryGraphLibrary>().graphs.insert(main_id.clone(), main_graph);

    // 3. Start Execution
    let mut executor = app.world_mut().resource_mut::<GraphExecutor>();
    executor.start(main_id, main_start);

    // 4. Update Loop
    // Iteration 1: Process m0 (SubGraph) -> Pushes to stack, Switches to sub_graph, Jumps to s0
    app.update();

    // Verify inside subgraph
    let flags = app.world().resource::<StoryFlags>();
    assert!(
        flags.get_bool("in_subgraph"),
        "Should be in subgraph and have processed s0"
    );

    // Iteration 2: Process s1 (End) -> Pops stack, Restores main_graph, Sets current to m0.next (1)
    app.update();

    // Iteration 3: Process m1 (SetFlag) -> Sets 'back_in_main', Jumps to 2
    app.update();

    let flags = app.world().resource::<StoryFlags>();
    assert!(
        flags.get_bool("back_in_main"),
        "Should have returned to main graph and processed m1"
    );

    let executor = app.world().resource::<GraphExecutor>();
    assert_eq!(executor.status, ExecutionStatus::Idle); // Should be done
}

#[test]
fn test_stack_depth() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app.add_plugins(AssetPlugin::default());
    app.add_plugins(bevy::state::app::StatesPlugin);
    app.add_plugins(bevy::input::InputPlugin);
    app.init_asset::<AudioSource>();
    app.add_plugins(DJEnginePlugin::default().without_diagnostics());

    // Create a recursive graph with Waits to pause execution
    // A -> B -> C (Wait)

    let mut graph_c = StoryGraph::new("C");
    let c_wait = graph_c.add(StoryNode::Wait {
        duration: 1.0,
        next: None,
    });
    let _c_end = graph_c.add(StoryNode::End);
    // Link wait to end (Wait next is None in test setup? No, should be Some(c_end))
    // Actually Wait { next: None } usually implies End or Stop?
    // advance_node uses next. If None, it ends graph.
    // So Wait with next: None works as "Wait then End".
    graph_c.set_start(c_wait);

    let mut graph_b = StoryGraph::new("B");
    let b0 = graph_b.add(StoryNode::SubGraph {
        graph_id: "C".into(),
        next: None,
    });
    graph_b.set_start(b0);

    let mut graph_a = StoryGraph::new("A");
    let a0 = graph_a.add(StoryNode::SubGraph {
        graph_id: "B".into(),
        next: None,
    });
    graph_a.set_start(a0);

    let mut library = app.world_mut().resource_mut::<StoryGraphLibrary>();
    library.graphs.insert("C".into(), graph_c);
    library.graphs.insert("B".into(), graph_b);
    library.graphs.insert("A".into(), graph_a);

    let mut executor = app.world_mut().resource_mut::<GraphExecutor>();
    executor.start("A".into(), Some(a0));

    // Frame 1: A -> B -> C -> Wait
    app.update();
    let executor = app.world().resource::<GraphExecutor>();
    // Stack should contain:
    // 1. (Graph A, None) - Pushed when entering B
    // 2. (Graph B, None) - Pushed when entering C
    assert_eq!(executor.stack.len(), 2, "Should be depth 2 (A->B->C)");
    assert_eq!(executor.status, ExecutionStatus::WaitingForTimer);

    // Manually tick the timer to ensure it finishes regardless of Bevy Time delta
    let mut executor = app.world_mut().resource_mut::<GraphExecutor>();
    executor
        .wait_timer
        .tick(std::time::Duration::from_secs_f32(1.1));

    // Frame 2: C finishes -> Pops B -> B finishes -> Pops A -> A finishes
    app.update();
    let executor = app.world().resource::<GraphExecutor>();
    assert_eq!(
        executor.stack.len(),
        0,
        "Stack should be empty after completion"
    );
    assert_eq!(executor.status, ExecutionStatus::Idle);
}
