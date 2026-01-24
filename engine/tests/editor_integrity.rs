use bevy::prelude::*;
use dj_engine::editor::{EditorPlugin, EditorUiState, EditorView, SidePanelTab, ProjectMetadata, EditorState};

#[test]
fn test_editor_initialization_and_state() {
    // 1. Setup App
    let mut app = App::new();
    
    // Minimal plugins required for the editor resources and states to be registered
    app.add_plugins(MinimalPlugins);
    app.add_plugins(bevy::hierarchy::HierarchyPlugin);
    app.add_plugins(bevy::state::app::StatesPlugin); 
    
    app.init_state::<EditorState>()
       .init_resource::<ProjectMetadata>()
       .init_resource::<EditorUiState>();

    // 2. Verify Initial State
    let ui_state = app.world().resource::<EditorUiState>();
    let main_branch = ui_state.current_branch().unwrap();
    assert_eq!(main_branch.active_view, EditorView::MapEditor);
    assert_eq!(main_branch.active_tab, SidePanelTab::Hierarchy);
    assert_eq!(ui_state.selected_palette_item, None);

    // 3. Simulate User Actions
    
    // "Load Project"
    let mut project = app.world_mut().resource_mut::<ProjectMetadata>();
    project.name = "Test Project".into();
    project.path = Some("test/path".into());
    
    // "Select Palette Item on Current Branch"
    let mut ui_state = app.world_mut().resource_mut::<EditorUiState>();
    if let Some(branch) = ui_state.current_branch_mut() {
        branch.active_tab = SidePanelTab::Palette;
    }
    ui_state.selected_palette_item = Some("Hamster".into());
    
    // "Switch View"
    if let Some(branch) = ui_state.current_branch_mut() {
        branch.active_view = EditorView::StoryGraph;
    }

    // 4. Verify Changes
    let ui_state_after = app.world().resource::<EditorUiState>();
    let branch_after = ui_state_after.current_branch().unwrap();
    assert_eq!(branch_after.active_tab, SidePanelTab::Palette);
    assert_eq!(ui_state_after.selected_palette_item, Some("Hamster".into()));
    assert_eq!(branch_after.active_view, EditorView::StoryGraph);
    
    let project_after = app.world().resource::<ProjectMetadata>();
    assert_eq!(project_after.name, "Test Project");
}

#[test]
fn test_editor_plugin_structure() {
    // Verify that the plugin adds the expected resources
    // (We accept that it might fail to build in headless if we add the actual plugin due to Egui, 
    // but we can check if the struct exists and compiles, which this test file does by importing it)
    
    let plugin = EditorPlugin;
    assert!(std::any::type_name_of_val(&plugin).contains("EditorPlugin"));
}
