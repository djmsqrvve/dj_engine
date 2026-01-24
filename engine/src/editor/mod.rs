//! Editor UI for DJ Engine.
//! 
//! Provides a professional game development environment using Egui.

pub mod state;
pub mod systems;
pub mod ui;
pub mod validation;

use bevy::prelude::*;
use bevy_egui::EguiPlugin;
pub use state::*;
use systems::*;
use ui::*;
use ui::campaign::CampaignEditorState;
use crate::data::EditorPreferences;

/// Resource wrapping the loaded EditorPreferences
#[derive(Resource)]
pub struct EditorPrefs(pub EditorPreferences);

pub struct EditorPlugin;

impl Plugin for EditorPlugin {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<EguiPlugin>() {
            app.add_plugins(EguiPlugin);
        }

        // Load user preferences from disk
        let mut preferences = EditorPreferences::load();
        info!("Loaded editor preferences from {:?}", EditorPreferences::default_path());

        // Argument Parsing
        let args: Vec<String> = std::env::args().collect();
        let mut initial_project = ProjectMetadata::default();
        let mut initial_view = EditorView::MapEditor; // Default to map editor
        let mut test_mode = false;

        let mut i = 0;
        while i < args.len() {
            match args[i].as_str() {
                "--project" => {
                    if i + 1 < args.len() {
                        initial_project.name = "Loaded from CLI".into();
                        initial_project.path = Some(args[i+1].clone().into());
                        info!("CLI: Pre-loading project from {}", args[i+1]);
                    }
                }
                "--view" => {
                    if i + 1 < args.len() {
                        initial_view = match args[i+1].as_str() {
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
                    initial_project.name = path.file_stem()
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

        app.init_state::<EditorState>()
            .insert_resource(EditorPrefs(preferences))
            .insert_resource(initial_project)
            .insert_resource(ui_state)
            .init_resource::<ActiveStoryGraph>()
            .init_resource::<ActiveMap>()
            .init_resource::<ActiveScenario>()
            .init_resource::<CampaignEditorState>()
            .init_resource::<EngineSettings>()
            .add_systems(Update, configure_visuals_system)
            .add_systems(Update, editor_ui_system)
            .add_systems(Update, apply_window_settings_system)
            .add_systems(OnEnter(EditorState::Playing), launch_project_system);
        
        if test_mode {
            app.insert_resource(AutomatedTestActive { 
                timer: Timer::from_seconds(0.5, TimerMode::Repeating),
                step: 0 
            })
            .add_systems(Update, automated_ui_test_system);
        }
        
        info!("DJ Engine Editor initialized");
    }
}
