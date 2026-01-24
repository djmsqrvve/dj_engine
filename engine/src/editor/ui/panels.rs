use crate::data::loader;
use crate::data::story::StoryNodeVariant;
use crate::diagnostics::console::ConsoleLogStore;
use bevy::prelude::*;
use bevy_egui::egui::{self, Color32, RichText};
use bevy_inspector_egui::bevy_inspector;
use std::path::PathBuf;

use super::super::state::*;
use super::super::systems::*;

pub fn draw_top_menu(ui: &mut egui::Ui, world: &mut World) {
    ui.vertical(|ui| {
        // ROW 1: APP MENU
        ui.horizontal(|ui| {
            // Logo with Cyberpunk colors
            ui.spacing_mut().item_spacing.x = 2.0;
            ui.label(
                RichText::new("DJ")
                    .color(COLOR_PRIMARY)
                    .strong()
                    .size(20.0)
                    .italics(),
            );
            ui.label(
                RichText::new("ENGINE")
                    .color(COLOR_SECONDARY)
                    .strong()
                    .size(20.0),
            );

            ui.add_space(10.0);
            ui.separator();
            ui.add_space(10.0);

            // FILE MENU
            ui.menu_button("File", |ui| {
                if ui.button("💾 Save Project").clicked() {
                    save_project_impl(world);
                    ui.close_menu();
                }
                if ui.button("📂 Load Project").clicked() {
                    let path = PathBuf::from("games/dev/new_horizon");
                    let mut project_meta = world.resource_mut::<ProjectMetadata>();
                    project_meta.path = Some(path.clone());

                    let project_file = path.join("project.json");
                    if project_file.exists() {
                        match loader::load_project(&project_file) {
                            Ok(project) => {
                                project_meta.name = project.name;
                                info!("Loaded project: {}", project_meta.name);
                            }
                            Err(e) => error!("Failed to load project.json: {}", e),
                        }
                    } else {
                        project_meta.name = "New Horizon".into();
                    }

                    let scene_path = path.join("scenes/intro_scene.json");
                    if scene_path.exists() {
                        match loader::load_scene(&scene_path) {
                            Ok(scene) => {
                                load_scene_into_editor(world, scene);
                                info!("Loaded scene: intro_scene");
                            }
                            Err(e) => error!("Failed to load scene: {}", e),
                        }
                    }

                    // Try load story graph
                    let graph_path = path.join("story_graphs/test_game.json");
                    if graph_path.exists() {
                        match loader::load_story_graph(&graph_path) {
                            Ok(graph) => {
                                world.insert_resource(ActiveStoryGraph(graph));
                                info!("Loaded story graph: test_game");
                            }
                            Err(e) => error!("Failed to load story graph: {}", e),
                        }
                    }

                    info!("Editor: Loaded project path 'games/dev/new_horizon'");
                    ui.close_menu();
                }
            });

            // EDIT MENU (Placeholder)
            ui.menu_button("Edit", |ui| {
                let _ = ui.button("Undo");
                let _ = ui.button("Redo");
                ui.separator();
                let _ = ui.button("Cut");
                let _ = ui.button("Copy");
                let _ = ui.button("Paste");
            });

            // VIEW MENU
            ui.menu_button("View", |ui| {
                let mut dock_state = world.resource_mut::<EditorDockState>();
                
                let tab_toggles = [
                    ("📁 Hierarchy", EditorView::Hierarchy),
                    ("🎨 Palette", EditorView::Palette),
                    ("📁 Assets", EditorView::Assets),
                    ("🔍 Inspector", EditorView::Inspector),
                    ("💻 Console", EditorView::Console),
                ];

                for (label, tab) in tab_toggles {
                    let is_open = dock_state.0.find_tab(&tab).is_some();
                    if ui.selectable_label(is_open, label).clicked() {
                        if is_open {
                            if let Some(tab_pos) = dock_state.0.find_tab(&tab) {
                                dock_state.0.remove_tab(tab_pos);
                            }
                        } else {
                            // We need to borrow the surface specifically here
                            dock_state.0.main_surface_mut().push_to_first_leaf(tab);
                        }
                    }
                }

                ui.separator();
                let _ = ui.button("Zoom In");
                let _ = ui.button("Zoom Out");
                ui.separator();

                let mut settings = world.resource_mut::<EngineSettings>();
                ui.checkbox(&mut settings.draw_grid, "Show Grid");
                ui.checkbox(&mut settings.show_bounds, "Show Bounds");
            });

            // TOOLS MENU (Placeholder)
            ui.menu_button("Tools", |ui| {
                let _ = ui.button("Asset Generator");
                let _ = ui.button("Validation");
            });

            // HELP MENU (Placeholder)
            ui.menu_button("Help", |ui| {
                let _ = ui.button("Documentation");
                let _ = ui.button("About DJ Engine");
            });
        });

        ui.add_space(2.0);
        ui.separator();
        ui.add_space(2.0);

        // ROW 2: NAVIGATION & CONTEXT (The "Pro Tool" Dashboard)
        // Vision: This row manages the *context* of what is being edited.
        // - CORE: Global project state and version control (Git-like).
        // - DASHBOARD: High-level visualization of all branches and their health.
        // - FEATURE GRID: Modular system toggles (Sphere Grid style).
        // - VIEWS: Specific domain editors (Spatial, Narrative, Logic).
        ui.horizontal(|ui| {
            let mut switch_branch_idx = None;
            let mut add_branch = false;
            let mut close_branch_idx = None;

            let (active_idx, is_core_global) = {
                let ui_state = world.resource::<EditorUiState>();
                (
                    ui_state.active_branch_idx,
                    ui_state.global_view == EditorView::Core,
                )
            };

            let branches = &world.resource::<EditorUiState>().active_branches;
            let active_branch = &branches[active_idx];
            let active_view = if is_core_global {
                EditorView::Core
            } else {
                active_branch.active_view.clone()
            };

            // 1. CORE / BRANCHES DROPDOWN
            ui.menu_button(
                RichText::new("📦 CORE")
                    .color(COLOR_PRIMARY)
                    .strong()
                    .size(14.0),
                |ui| {
                    ui.label("Branches");
                    ui.separator();
                    for (idx, branch) in branches.iter().enumerate() {
                        let is_active = idx == active_idx;
                        let text = if is_active {
                            format!("● {}", branch.name)
                        } else {
                            branch.name.clone()
                        };

                        ui.horizontal(|ui| {
                            if ui.selectable_label(is_active, text).clicked() {
                                switch_branch_idx = Some(idx);
                                ui.close_menu();
                            }
                            if branches.len() > 1 && ui.small_button("x").clicked() {
                                close_branch_idx = Some(idx);
                                ui.close_menu();
                            }
                        });
                    }
                    ui.separator();
                    if ui.button("+ New Branch").clicked() {
                        add_branch = true;
                        ui.close_menu();
                    }
                },
            );

            ui.add_space(5.0);
            ui.separator();
            ui.add_space(5.0);

            // 2. NAVIGATION TABS
            if ui
                .selectable_label(is_core_global, "📋 Core Dashboard")
                .clicked()
            {
                if let Some(mut state) = world.get_resource_mut::<EditorUiState>() {
                    state.global_view = EditorView::Core;
                }
            }
            ui.add_space(5.0);

            let is_feature_grid = active_view == EditorView::FeatureGrid;
            if ui
                .selectable_label(is_feature_grid, "🔷 Feature Grid")
                .clicked()
            {
                if let Some(mut state) = world.get_resource_mut::<EditorUiState>() {
                    state.global_view = EditorView::FeatureGrid;
                    if let Some(branch) = state.active_branches.get_mut(active_idx) {
                        branch.active_view = EditorView::FeatureGrid;
                    }
                }
            }
            ui.add_space(5.0);

            let is_timeline = active_view == EditorView::Timeline;
            if ui.selectable_label(is_timeline, "🎹 Timeline").clicked() {
                if let Some(mut state) = world.get_resource_mut::<EditorUiState>() {
                    state.global_view = EditorView::Timeline;
                    if let Some(branch) = state.active_branches.get_mut(active_idx) {
                        branch.active_view = EditorView::Timeline;
                    }
                }
            }
            ui.add_space(10.0);

            let views = [
                (EditorView::MapEditor, "🗺 Map"),
                (EditorView::ScenarioEditor, "🎭 Scenario"),
                (EditorView::StoryGraph, "📽 Storyboard"),
                (EditorView::Campaign, "📅 Campaign"),
                (EditorView::Play, "🎮 Play"),
                (EditorView::Settings, "⚙ Settings"),
            ];

            for (view, label) in views {
                let is_selected = !is_core_global && active_view == view;
                if ui.selectable_label(is_selected, label).clicked() {
                    if let Some(mut state) = world.get_resource_mut::<EditorUiState>() {
                        state.global_view = view.clone(); // Any non-Core view disables dashboard
                        if let Some(branch) = state.active_branches.get_mut(active_idx) {
                            branch.active_view = view;
                        }
                    }
                }
                ui.add_space(5.0);
            }

            // Handle branch mutations
            if let Some(idx) = switch_branch_idx {
                world.resource_mut::<EditorUiState>().active_branch_idx = idx;
            }

            if add_branch {
                let count = world.resource::<EditorUiState>().active_branches.len() + 1;
                let color = match count % 4 {
                    0 => COLOR_PRIMARY,
                    1 => COLOR_SECONDARY,
                    2 => Color32::from_rgb(100, 200, 255),
                    _ => Color32::from_rgb(255, 200, 100),
                };

                world
                    .resource_mut::<EditorUiState>()
                    .active_branches
                    .push(Branch {
                        id: uuid::Uuid::new_v4().to_string(),
                        name: format!("Branch {}", count),
                        color,
                        active_view: EditorView::MapEditor,
                        active_tab: SidePanelTab::Hierarchy,
                        history: vec![],
                        node_overrides: std::collections::HashMap::new(),
                    });
                let new_idx = world.resource::<EditorUiState>().active_branches.len() - 1;
                world.resource_mut::<EditorUiState>().active_branch_idx = new_idx;
            }

            if let Some(idx) = close_branch_idx {
                let mut state = world.resource_mut::<EditorUiState>();
                if state.active_branches.len() > 1 {
                    state.active_branches.remove(idx);
                    if state.active_branch_idx >= state.active_branches.len() {
                        state.active_branch_idx = state.active_branches.len() - 1;
                    }
                }
            }
        });
    });
}

pub fn draw_left_panel(ui: &mut egui::Ui, world: &mut World) {
    ui.add_space(5.0);

    let current_tab = {
        let state = world.resource::<EditorUiState>();
        state
            .current_branch()
            .map(|b| b.active_tab)
            .unwrap_or_default()
    };

    let mut selected_tab = current_tab;

    ui.horizontal(|ui| {
        ui.style_mut().visuals.selection.bg_fill = COLOR_PRIMARY.linear_multiply(0.2);
        if ui
            .selectable_label(selected_tab == SidePanelTab::Hierarchy, "Hierarchy")
            .clicked()
        {
            selected_tab = SidePanelTab::Hierarchy;
        }
        if ui
            .selectable_label(selected_tab == SidePanelTab::Palette, "Palette")
            .clicked()
        {
            selected_tab = SidePanelTab::Palette;
        }
        if ui
            .selectable_label(selected_tab == SidePanelTab::Assets, "Assets")
            .clicked()
        {
            selected_tab = SidePanelTab::Assets;
        }
    });

    if selected_tab != current_tab {
        if let Some(branch) = world.resource_mut::<EditorUiState>().current_branch_mut() {
            branch.active_tab = selected_tab;
        }
    }

    ui.add_space(5.0);
    ui.separator();
    ui.add_space(5.0);

    match selected_tab {
        SidePanelTab::Hierarchy => {
            ui.label(
                RichText::new("SCENE HIERARCHY")
                    .strong()
                    .color(COLOR_PRIMARY),
            );
            ui.add_space(5.0);

            egui::ScrollArea::vertical().show(ui, |ui| {
                world.resource_scope::<EditorUiState, _>(|world, mut ui_state| {
                    bevy_inspector::hierarchy::hierarchy_ui(
                        world,
                        ui,
                        &mut ui_state.selected_entities,
                    );
                });
            });
        }
        SidePanelTab::Palette => {
            ui.label(RichText::new("NODE DROPPER").strong().color(COLOR_PRIMARY));
            ui.add_space(5.0);

            egui::ScrollArea::vertical().show(ui, |ui| {
                // Variables Section
                ui.label(RichText::new("Variables").strong());
                ui.horizontal(|ui| {
                    ui.button("123 Integer").clicked();
                    if ui.button("1.23 Float").clicked() {}
                });
                ui.horizontal(|ui| {
                    ui.button("☑ Boolean").clicked();
                    if ui.button("➡ Vector3").clicked() {}
                });
                ui.separator();

                // Conditions Section
                ui.label(RichText::new("Conditions").strong());
                ui.horizontal(|ui| {
                    ui.button("❓ If").clicked();
                    if ui.button("⚖ Compare").clicked() {}
                });
                let _ = ui.button("🔀 Switch");
                ui.separator();

                // Events
                ui.label(RichText::new("Events").strong());
                ui.horizontal(|ui| {
                    ui.button("🚩 On Trigger").clicked();
                    if ui.button("🕓 On Timer").clicked() {}
                });
                let _ = ui.button("💥 On Damage");
                ui.separator();

                // Actions
                ui.label(RichText::new("Actions").strong());
                ui.button("📦 Set Variable").clicked();
                ui.button("🔊 Play Sound").clicked();
                if ui.button("⚔ Create Unit").clicked() {}
            });
        }
        SidePanelTab::Assets => {
            draw_assets_browser(ui, world);
        }
    }
}

fn draw_assets_browser(ui: &mut egui::Ui, _world: &mut World) {
    ui.label(RichText::new("ASSET BROWSER").strong().color(COLOR_PRIMARY));
    ui.add_space(5.0);
    ui.label(
        RichText::new("Coming soon...")
            .italics()
            .color(Color32::GRAY),
    );
}

pub fn draw_right_panel(ui: &mut egui::Ui, world: &mut World) {
    ui.add_space(5.0);
    ui.label(RichText::new("INSPECTOR").strong().color(COLOR_PRIMARY));
    ui.add_space(5.0);
    ui.separator();

    // Check if we are in Story Graph mode and have a selected node
    let story_node_selected = {
        let state = world.resource::<EditorUiState>();
        let current_view = state
            .current_branch()
            .map(|b| b.active_view.clone())
            .unwrap_or_default();

        if current_view == EditorView::StoryGraph {
            state.selected_node_id.clone()
        } else {
            None
        }
    };

    if let Some(node_id) = story_node_selected {
        // Edit Story Node
        world.resource_scope::<ActiveStoryGraph, _>(|_, mut graph| {
            if let Some(node) = graph.0.nodes.iter_mut().find(|n| n.id == node_id) {
                ui.label(RichText::new(format!("Node: {}", node.id)).strong());
                ui.separator();

                ui.label("Position");
                ui.horizontal(|ui| {
                    ui.label("X:");
                    ui.add(egui::DragValue::new(&mut node.position.x));
                    ui.label("Y:");
                    ui.add(egui::DragValue::new(&mut node.position.y));
                });

                ui.separator();
                ui.label("Properties");

                match &mut node.data {
                    StoryNodeVariant::Start(_) => {
                        ui.label("Start Node (Entry Point)");
                    }
                    StoryNodeVariant::Dialogue(d) => {
                        ui.label("Speaker:");
                        ui.text_edit_singleline(&mut d.speaker_id);
                        ui.label("Text (EN):");
                        let mut text = d.text.get("en").cloned().unwrap_or_default();
                        if ui.text_edit_multiline(&mut text).changed() {
                            d.text.insert("en".to_string(), text);
                        }
                    }
                    StoryNodeVariant::End(e) => {
                        ui.label("Target Scene ID:");
                        let mut scene = e.target_scene_id.clone().unwrap_or_default();
                        if ui.text_edit_singleline(&mut scene).changed() {
                            e.target_scene_id = if scene.is_empty() { None } else { Some(scene) };
                        }
                    }
                    _ => {
                        ui.label("Not implemented in inspector yet.");
                    }
                }
            }
        });
        return;
    }

    world.resource_scope::<EditorUiState, _>(|world, ui_state| {
        if ui_state.selected_entities.is_empty() {
            ui.add_space(10.0);
            ui.label(
                RichText::new("No entity selected.")
                    .italics()
                    .color(Color32::GRAY),
            );
            ui.add_space(10.0);
            ui.separator();
            ui.collapsing("Global Resources", |ui| {
                bevy_inspector::ui_for_resources(world, ui);
            });
        } else {
            // Show detailed info for each selected entity
            for &entity in ui_state.selected_entities.as_slice() {
                ui.add_space(5.0);

                // Entity header with ID and generation
                let gen = entity.generation();
                let index = entity.index();
                let name = world
                    .get::<Name>(entity)
                    .map(|n| n.as_str().to_string())
                    .unwrap_or_else(|| "<unnamed>".to_string());

                ui.group(|ui| {
                    ui.horizontal(|ui| {
                        ui.label(RichText::new("🔷").size(16.0));
                        ui.label(
                            RichText::new(&name)
                                .strong()
                                .color(COLOR_PRIMARY)
                                .size(14.0),
                        );
                    });
                    ui.horizontal(|ui| {
                        ui.label(RichText::new("Entity ID:").color(Color32::GRAY));
                        ui.label(
                            RichText::new(format!("{}v{}", index, gen))
                                .monospace()
                                .color(COLOR_SECONDARY),
                        );
                        if gen > 10 {
                            ui.label(RichText::new("⚠️ High gen!").color(Color32::YELLOW).small());
                        }
                    });

                    // Show Metadata if available
                    if let Some(meta) = world.get::<crate::data::components::EntityMetadata>(entity)
                    {
                        ui.separator();
                        ui.horizontal(|ui| {
                            ui.label(RichText::new("Creator:").color(Color32::GRAY));
                            ui.label(RichText::new(&meta.creator_id).monospace());
                        });
                        ui.horizontal(|ui| {
                            ui.label(RichText::new("Created:").color(Color32::GRAY));
                            // Format timestamp using chrono
                            let datetime =
                                chrono::DateTime::from_timestamp(meta.creation_timestamp as i64, 0);
                            let time_str = if let Some(dt) = datetime {
                                dt.format("%Y-%m-%d %H:%M:%S").to_string()
                            } else {
                                format!("{:.0}", meta.creation_timestamp)
                            };
                            ui.label(RichText::new(time_str).monospace());
                        });
                    }
                });

                ui.separator();
            }

            // Show components for selected entities
            ui.collapsing(RichText::new("📦 Components").strong(), |ui| {
                bevy_inspector::ui_for_entities_shared_components(
                    world,
                    ui_state.selected_entities.as_slice(),
                    ui,
                );
            })
            .header_response
            .on_hover_text("Inspect and edit component values");
        }
    });
}

pub fn draw_console_panel(ui: &mut egui::Ui, world: &mut World) {
    use crate::diagnostics::console::ConsoleCommandEvent;

    // Header row
    ui.horizontal(|ui| {
        ui.label(RichText::new("💻 CONSOLE").color(COLOR_PRIMARY).strong());
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if ui
                .button(RichText::new("Clear").color(Color32::GRAY))
                .clicked()
            {
                if let Some(mut store) = world.get_resource_mut::<ConsoleLogStore>() {
                    store.logs.clear();
                }
            }
        });
    });
    ui.separator();

    // Log output area (takes most of the space)
    let available_height = ui.available_height() - 30.0; // Reserve space for input
    egui::ScrollArea::vertical()
        .max_height(available_height.max(50.0))
        .stick_to_bottom(true)
        .show(ui, |ui| {
            if let Some(store) = world.get_resource::<ConsoleLogStore>() {
                for log in &store.logs {
                    let color = if log.contains("TEST:") || log.contains("Passed") {
                        COLOR_PRIMARY
                    } else if log.contains("WARN") {
                        COLOR_SECONDARY
                    } else if log.contains("ERROR") {
                        Color32::RED
                    } else {
                        Color32::LIGHT_GRAY
                    };
                    ui.label(RichText::new(log).color(color).monospace());
                }
            } else {
                ui.label("ConsoleLogStore resource missing.");
            }
        });

    ui.separator();

    // Input row
    let mut submit_command = false;
    let mut command_to_send = String::new();

    ui.horizontal(|ui| {
        ui.label(RichText::new("dj>").color(COLOR_PRIMARY).monospace());

        let input_response = {
            let mut ui_state = world.resource_mut::<EditorUiState>();
            let response = ui.add(
                egui::TextEdit::singleline(&mut ui_state.console_input)
                    .desired_width(ui.available_width() - 60.0)
                    .font(egui::TextStyle::Monospace)
                    .hint_text("Type a command (help for list)..."),
            );
            response
        };

        // Check for Enter key
        if input_response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
            submit_command = true;
            command_to_send = world.resource::<EditorUiState>().console_input.clone();
        }

        if ui
            .button(RichText::new("Run").color(COLOR_PRIMARY))
            .clicked()
        {
            submit_command = true;
            command_to_send = world.resource::<EditorUiState>().console_input.clone();
        }
    });

    // Process command submission
    if submit_command && !command_to_send.is_empty() {
        // Log to console output
        if let Some(mut store) = world.get_resource_mut::<ConsoleLogStore>() {
            store.log(format!("> {}", command_to_send));
        }

        // Send command event
        world.send_event(ConsoleCommandEvent(command_to_send));

        // Clear input
        world.resource_mut::<EditorUiState>().console_input.clear();
    }
}

pub fn draw_palette_content(ui: &mut egui::Ui) {
    ui.label(RichText::new("NODE DROPPER").strong().color(COLOR_PRIMARY));
    ui.add_space(5.0);

    egui::ScrollArea::vertical().show(ui, |ui| {
        // Variables Section
        ui.label(RichText::new("Variables").strong());
        ui.horizontal(|ui| {
            ui.button("123 Integer").clicked();
            if ui.button("1.23 Float").clicked() {}
        });
        ui.horizontal(|ui| {
            ui.button("☑ Boolean").clicked();
            if ui.button("➡ Vector3").clicked() {}
        });
        ui.separator();

        // Conditions Section
        ui.label(RichText::new("Conditions").strong());
        ui.horizontal(|ui| {
            ui.button("❓ If").clicked();
            if ui.button("⚖ Compare").clicked() {}
        });
        let _ = ui.button("🔀 Switch");
        ui.separator();

        // Events
        ui.label(RichText::new("Events").strong());
        ui.horizontal(|ui| {
            ui.button("🚩 On Trigger").clicked();
            if ui.button("🕓 On Timer").clicked() {}
        });
        let _ = ui.button("💥 On Damage");
        ui.separator();

        // Actions
        ui.label(RichText::new("Actions").strong());
        ui.button("📦 Set Variable").clicked();
        ui.button("🔊 Play Sound").clicked();
        if ui.button("⚔ Create Unit").clicked() {}
    });
}
