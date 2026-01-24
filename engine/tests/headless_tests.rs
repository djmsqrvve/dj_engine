use bevy::app::ScheduleRunnerPlugin;
use bevy::prelude::*;
use dj_engine::editor::state::{EditorUiState, EditorView, FeatureGrid};
use dj_engine::prelude::*;
#[test]
fn test_headless_initialization() {
    let mut app = App::new();

    // Add MinimalPlugins (no window, no renderer)
    app.add_plugins(MinimalPlugins.set(ScheduleRunnerPlugin::run_once()));
    app.add_plugins(bevy::state::app::StatesPlugin);
    app.add_plugins(bevy::asset::AssetPlugin::default());
    app.add_plugins(bevy::audio::AudioPlugin::default());
    app.add_plugins(bevy::input::InputPlugin::default());

    // Add Core Engine Plugin
    app.add_plugins(DJEnginePlugin::default());

    // Manually init editor resources instead of EditorPlugin (avoids Egui requirements)
    app.init_resource::<EditorUiState>();
    app.init_resource::<FeatureGrid>();

    // Update once
    app.update();

    // Verify EditorUIState resource exists
    assert!(
        app.world().contains_resource::<EditorUiState>(),
        "EditorUiState should be initialized"
    );

    // Verify FeatureGrid resource exists
    assert!(
        app.world().contains_resource::<FeatureGrid>(),
        "FeatureGrid should be initialized"
    );

    // Check initial state
    let ui_state = app.world().resource::<EditorUiState>();
    assert_eq!(
        ui_state.global_view,
        EditorView::Core,
        "Default global view should be Core"
    );
}

#[test]
fn test_feature_grid_population() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins.set(ScheduleRunnerPlugin::run_once()));
    app.add_plugins(bevy::state::app::StatesPlugin);
    app.add_plugins(bevy::asset::AssetPlugin::default());
    app.add_plugins(bevy::audio::AudioPlugin::default());
    app.add_plugins(bevy::input::InputPlugin::default());
    app.add_plugins(DJEnginePlugin::default());
    app.init_resource::<EditorUiState>();
    app.init_resource::<FeatureGrid>();
    app.update();

    let grid = app.world().resource::<FeatureGrid>();

    // Verify ecosystems
    assert!(
        !grid.ecosystems.is_empty(),
        "Feature Grid should have ecosystems"
    );

    // Check for specific ecosystems
    let has_abilities = grid.ecosystems.iter().any(|e| e.id == "abilities");
    let has_items = grid.ecosystems.iter().any(|e| e.id == "items");

    assert!(has_abilities, "Should have 'abilities' ecosystem");
    assert!(has_items, "Should have 'items' ecosystem");
}

#[test]
fn test_story_graph_loading() {
    use dj_engine::data::loader;
    use std::path::PathBuf;

    // Path to the test game we created
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.pop(); // Up to root (dj_engine)
    path.push("games/dev/new_horizon/story_graphs/test_game.json");

    println!("Loading story graph from: {:?}", path);
    assert!(path.exists(), "Test game JSON must exist at {:?}", path);

    // Attempt load
    let result = loader::load_story_graph(&path);
    assert!(result.is_ok(), "Should successfully load test_game.json");

    let graph = result.unwrap();
    assert_eq!(graph.id, "test_game", "Graph ID should match");
    assert!(!graph.nodes.is_empty(), "Graph should have nodes");

    // Verify Start Node
    // Using simple lookup as fallback if field name varies
    let start_node = graph.nodes.iter().find(|n| n.id == "1");
    assert!(start_node.is_some(), "Start node '1' should exist");
}
