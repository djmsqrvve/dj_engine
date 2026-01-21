//! Editor UI for DJ Engine.
//! 
//! Provides a professional game development environment using Egui.

use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum EditorState {
    #[default]
    Editor,
    Playing,
}

/// Resource holding the current project metadata.
#[derive(Resource, Default)]
pub struct ProjectMetadata {
    pub name: String,
    pub path: Option<std::path::PathBuf>,
}

pub struct EditorPlugin;

impl Plugin for EditorPlugin {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<EguiPlugin>() {
            app.add_plugins(EguiPlugin);
        }

        app.init_state::<EditorState>()
            .init_resource::<ProjectMetadata>()
            .add_systems(Update, editor_ui_system)
            .add_systems(OnEnter(EditorState::Playing), launch_project_system);
        
        info!("DJ Engine Editor initialized");
    }
}

fn launch_project_system(
    project: Res<ProjectMetadata>,
    mut script_events: EventWriter<crate::scripting::ScriptCommand>,
) {
    let Some(path) = &project.path else { 
        warn!("No project path mounted! Cannot launch.");
        return; 
    };

    info!("Editor: Launching project from {:?}", path);
    
    // Look for a main.lua or hamster_test.lua in the project's script folder
    let script_path = path.join("assets/scripts/hamster_test.lua");
    if script_path.exists() {
        script_events.send(crate::scripting::ScriptCommand::Load { 
            path: script_path.to_string_lossy().into() 
        });
    } else {
        warn!("No entry script found at {:?}", script_path);
    }
}

fn editor_ui_system(
    mut contexts: EguiContexts,
    mut next_state: ResMut<NextState<EditorState>>,
    current_state: Res<State<EditorState>>,
    mut project: ResMut<ProjectMetadata>,
) {
    let ctx = contexts.ctx_mut();

    egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
        ui.horizontal(|ui| {
            ui.heading("DJ ENGINE EDITOR");
            ui.separator();
            
            let is_playing = *current_state.get() == EditorState::Playing;
            
            if ui.add_enabled(!is_playing, egui::Button::new("▶ Play")).clicked() {
                next_state.set(EditorState::Playing);
                info!("Launching Project: {:?}", project.name);
            }
            if ui.add_enabled(is_playing, egui::Button::new("⏹ Stop")).clicked() {
                next_state.set(EditorState::Editor);
                info!("Stopping Project");
            }
            
            ui.separator();
            if ui.button("📁 Load").clicked() {
                // In a real editor this would open a file picker
                // For now, let's hardcode for the demo
                project.name = "DoomExe".into();
                project.path = Some("games/dev/doomexe".into());
                info!("Loaded Project: DoomExe");
            }
            
            ui.separator();
            if ui.button("🛠 Build").clicked() {
                info!("Building Project...");
            }

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.label(format!("Project: {}", project.name));
            });
        });
    });

    egui::SidePanel::left("hierarchy_panel")
        .default_width(200.0)
        .show(ctx, |ui| {
            ui.heading("Hierarchy");
            ui.separator();
            ui.label("• Player");
            ui.label("  • MainCamera");
            ui.label("  • HamsterScript");
            ui.label("• ForestMap");
        });

    egui::SidePanel::right("inspector_panel")
        .default_width(250.0)
        .show(ctx, |ui| {
            ui.heading("Inspector");
            ui.separator();
            ui.collapsing("Transform", |ui| {
                ui.horizontal(|ui| {
                    ui.label("Position");
                    let mut x = 0.0;
                    ui.add(egui::DragValue::new(&mut x));
                    ui.add(egui::DragValue::new(&mut x));
                });
            });
        });

    egui::TopBottomPanel::bottom("bottom_panel")
        .default_height(150.0)
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                let _ = ui.selectable_label(true, "Console");
                let _ = ui.selectable_label(false, "Assets");
            });
            ui.separator();
            ui.label("[12:45:10] INFO: Engine started.");
            ui.label("[12:45:11] INFO: Hamster spawned.");
            if project.path.is_some() {
                ui.label(format!("[12:47:00] INFO: Project mounted at {:?}", project.path.as_ref().unwrap()));
            }
        });

    egui::CentralPanel::default().show(ctx, |ui| {
        if *current_state.get() == EditorState::Editor {
            ui.centered_and_justified(|ui| {
                ui.label("VIEWPORT (Editor Mode)");
            });
        } else {
            ui.centered_and_justified(|ui| {
                ui.label("GAME IS RUNNING...");
            });
        }
    });
}
