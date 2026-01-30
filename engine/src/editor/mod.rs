//! Editor UI for DJ Engine.
//!
//! Provides a professional game development environment using Egui.

pub mod state;
pub mod systems;
pub mod ui;
pub mod validation;

use crate::data::EditorPreferences;
use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use egui_dock::{DockState, NodeIndex}; // Add imports
pub use state::*;
use systems::*;
use ui::campaign::CampaignEditorState;
use ui::*;

// EditorPrefs removed here, it's defined in state.rs

pub struct EditorPlugin;

impl Plugin for EditorPlugin {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<EguiPlugin>() {
            app.add_plugins(EguiPlugin);
        }

        // Load user preferences from disk
        let mut preferences = EditorPreferences::load();
        info!(
            "Loaded editor preferences from {:?}",
            EditorPreferences::default_path()
        );

        // Argument Parsing
        let args: Vec<String> = std::env::args().collect();
        let mut initial_project = ProjectMetadata::default();
        let mut initial_view = EditorView::MapEditor; // Default to map editor
        let mut test_mode = false;
        let mut start_playing = false;

        let mut i = 0;
        while i < args.len() {
            match args[i].as_str() {
                "--project" => {
                    if i + 1 < args.len() {
                        initial_project.name = "Loaded from CLI".into();
                        initial_project.path = Some(args[i + 1].clone().into());
                        info!("CLI: Pre-loading project from {}", args[i + 1]);
                    }
                }
                "--view" => {
                    if i + 1 < args.len() {
                        initial_view = match args[i + 1].as_str() {
                            "story" => EditorView::StoryGraph,
                            "scenario" => EditorView::ScenarioEditor,
                            _ => EditorView::MapEditor,
                        };
                        info!("CLI: Setting initial view to {:?}", initial_view);
                    }
                }
                "--test-mode" => {
                    test_mode = true;
                    info!("CLI: Automated Test Mode Enabled");
                }
                "--play" => {
                    start_playing = true;
                    info!("CLI: Direct Play Mode Enabled");
                }
                _ => {}
            }
            i += 1;
        }

        // If no CLI project specified and load_last_project is enabled, load last project
        if initial_project.path.is_none() && preferences.load_last_project {
            if let Some(last) = preferences.last_project().map(|s| s.to_owned()) {
                let path = std::path::PathBuf::from(&last);
                if path.exists() {
                    info!("Auto-loading last project: {}", last);
                    initial_project.name = path
                        .file_stem()
                        .and_then(|s| s.to_str())
                        .unwrap_or("Project")
                        .to_string();
                    initial_project.path = Some(path);
                } else {
                    warn!("Last project path no longer exists: {}", last);
                    // Clean up invalid path from recent projects
                    preferences.recent_projects.retain(|p| p != &last);
                    let _ = preferences.save();
                }
            }
        }

        let mut ui_state = EditorUiState::new();
        // Override initial branch view if specified
        if let Some(branch) = ui_state.current_branch_mut() {
            branch.active_view = initial_view;
        }

        // Initialize DockState
        let mut dock_state = if let Some(json) = &preferences.dock_state {
            match serde_json::from_value::<DockState<EditorView>>(json.clone()) {
                Ok(state) => {
                    info!("Restored dock layout from preferences");
                    state
                }
                Err(e) => {
                    warn!("Failed to restore dock layout: {}. Using default.", e);
                    DockState::new(vec![EditorView::StoryGraph])
                }
            }
        } else {
            DockState::new(vec![EditorView::StoryGraph])
        };

        // Ensure default layout if empty (or reset if needed) - simplified for now:
        // If we loaded a fresh one or default, we might want to apply the splits if it's the simple default.
        // But checking if it's "simple default" is hard. Let's assume if we failed to load or it was None, we apply defaults.
        if preferences.dock_state.is_none() {
            let surface = dock_state.main_surface_mut();
            let [center, _right] = surface.split_right(
                NodeIndex::root(),
                0.75,
                vec![EditorView::Inspector, EditorView::Settings],
            );
            let [_left, _center] = surface.split_left(
                center,
                0.2,
                vec![EditorView::Hierarchy, EditorView::Palette],
            );
            let [_center_top, _bottom] =
                surface.split_below(_center, 0.7, vec![EditorView::Console, EditorView::Assets]);
            surface.push_to_first_leaf(EditorView::Core);
        }

        if start_playing {
            app.insert_state(EditorState::Playing);
        } else {
            app.insert_state(EditorState::Editor);
        }

        app.register_type::<Commit>()
            .register_type::<CommitStatus>()
            .register_type::<Branch>()
            .register_type::<EditorView>()
            .register_type::<SidePanelTab>()
            .insert_resource(EditorPrefs(preferences))
            .insert_resource(initial_project)
            .insert_resource(ui_state)
            .insert_resource(EditorDockState(dock_state)) // Insert DockState
            .init_resource::<ActiveStoryGraph>()
            .init_resource::<ActiveMap>()
            .init_resource::<ActiveScenario>()
            .init_resource::<CampaignEditorState>()
            .insert_resource(EngineSettings::load())
            .init_resource::<FeatureGrid>()
            .add_systems(Update, configure_visuals_system)
            .add_systems(Update, editor_ui_system)
            .add_systems(Update, apply_window_settings_system)
            .add_systems(Update, sync_dock_layout_system) // Sync Dock
            .add_systems(Update, auto_save_prefs_system) // Auto Save
            .add_systems(Startup, cli_load_startup_system)
            .add_systems(OnEnter(EditorState::Playing), launch_project_system);

        if test_mode {
            app.insert_resource(AutomatedTestActive {
                timer: Timer::from_seconds(0.5, TimerMode::Repeating),
                step: 0,
            })
            .add_systems(Update, automated_ui_test_system);
        }

        info!("DJ Engine Editor initialized");
    }
}
