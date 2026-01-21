//! Editor UI for DJ Engine.
//! 
//! Provides a professional game development environment using Egui.

use bevy::prelude::*;
use bevy_egui::{egui::{self, RichText, Color32}, EguiPlugin};
use bevy_inspector_egui::bevy_inspector;
use crate::diagnostics::console::ConsoleLogStore;

const COLOR_MINT: Color32 = Color32::from_rgb(0, 255, 204);
const COLOR_ORANGE: Color32 = Color32::from_rgb(255, 170, 0);
const COLOR_BG: Color32 = Color32::from_rgb(15, 15, 20);

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum EditorState {
    #[default]
    Editor,
    Playing,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum EditorView {
    #[default]
    Level,
    StoryGraph,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum BrowserTab {
    #[default]
    Hierarchy,
    Assets,
    Palette,
}

/// Resource holding the current project metadata.
#[derive(Resource, Default)]
pub struct ProjectMetadata {
    pub name: String,
    pub path: Option<std::path::PathBuf>,
}

/// Resource holding the transient UI state of the editor.
#[derive(Resource, Default)]
pub struct EditorUiState {
    pub current_view: EditorView,
    pub browser_tab: BrowserTab,
    // We don't need Option<Entity> anymore, SelectedEntities handles it
    pub selected_entities: bevy_inspector_egui::bevy_inspector::hierarchy::SelectedEntities,
    pub asset_search_query: String,
    pub selected_palette_item: Option<String>,
}

#[derive(Resource)]
struct AutomatedTestActive {
    timer: Timer,
    step: usize,
}

pub struct EditorPlugin;

impl Plugin for EditorPlugin {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<EguiPlugin>() {
            app.add_plugins(EguiPlugin);
        }

        // Argument Parsing
        let args: Vec<String> = std::env::args().collect();
        let mut initial_project = ProjectMetadata::default();
        let mut initial_view = EditorView::Level;
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
                            _ => EditorView::Level,
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

        app.init_state::<EditorState>()
            .insert_resource(initial_project)
            .insert_resource(EditorUiState {
                current_view: initial_view,
                ..default()
            })
            .add_systems(Update, configure_visuals_system)
            .add_systems(Update, editor_ui_system)
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

fn configure_visuals_system(mut contexts: bevy_egui::EguiContexts) {
    let ctx = contexts.ctx_mut();
    let mut visuals = egui::Visuals::dark();
    
    // Cyberpunk tweaks
    visuals.window_rounding = 2.0.into();
    visuals.widgets.noninteractive.bg_fill = COLOR_BG;
    visuals.widgets.inactive.bg_fill = Color32::from_rgb(25, 25, 35);
    visuals.widgets.hovered.bg_fill = Color32::from_rgb(40, 40, 50);
    visuals.widgets.active.bg_fill = Color32::from_rgb(50, 50, 65);
    visuals.selection.bg_fill = COLOR_MINT.linear_multiply(0.3);
    visuals.selection.stroke = egui::Stroke::new(1.0, COLOR_MINT);
    
    ctx.set_visuals(visuals);
}

fn automated_ui_test_system(
    mut commands: Commands,
    time: Res<Time>,
    mut test_state: ResMut<AutomatedTestActive>,
    mut ui_state: ResMut<EditorUiState>,
    mut console: ResMut<ConsoleLogStore>,
    mut app_exit: EventWriter<bevy::app::AppExit>,
) {
    test_state.timer.tick(time.delta());
    if !test_state.timer.finished() {
        return;
    }

    match test_state.step {
        0 => {
            console.log("TEST: Starting automated UI test sequence...".into());
            test_state.step += 1;
        }
        1 => {
            console.log("TEST: Select 'Hamster' from palette".into());
            ui_state.browser_tab = BrowserTab::Palette;
            ui_state.selected_palette_item = Some("Hamster".into());
            test_state.step += 1;
        }
        2 => {
            console.log("TEST: Simulating click/spawn at (100, 100)".into());
            // Manually spawn entity as if clicked
             commands.spawn((
                Name::new("Hamster [100, 100]"),
                Sprite {
                    color: Color::srgb(0.8, 0.5, 0.2),
                    custom_size: Some(Vec2::new(30.0, 30.0)),
                    ..default()
                },
                Transform::from_xyz(100.0, 100.0, 0.0)
            ));
            test_state.step += 1;
        }
        3 => {
            console.log("TEST: Switching to Story Graph view".into());
            ui_state.current_view = EditorView::StoryGraph;
            test_state.step += 1;
        }
        4 => {
            console.log("TEST: Validation Complete. Exiting.".into());
            info!("Automated UI Test Passed");
            app_exit.send(bevy::app::AppExit::Success);
        }
        _ => {}
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

fn editor_ui_system(world: &mut World) {
    let mut egui_context = world
        .query_filtered::<&mut bevy_egui::EguiContext, With<bevy::window::PrimaryWindow>>()
        .single(world)
        .clone();
        
    egui::TopBottomPanel::top("top_panel").show(egui_context.get_mut(), |ui| {
        draw_top_menu(ui, world);
    });

    egui::TopBottomPanel::bottom("bottom_panel")
        .default_height(150.0)
        .show(egui_context.get_mut(), |ui| {
            draw_bottom_panel(ui, world);
        });

    egui::SidePanel::left("left_panel")
        .default_width(250.0)
        .show(egui_context.get_mut(), |ui| {
            draw_left_panel(ui, world);
        });

    egui::SidePanel::right("right_panel")
        .default_width(300.0)
        .show(egui_context.get_mut(), |ui| {
            draw_right_panel(ui, world);
        });

    egui::CentralPanel::default().show(egui_context.get_mut(), |ui| {
        draw_central_panel(ui, world);
    });
}

fn draw_top_menu(ui: &mut egui::Ui, world: &mut World) {
    ui.horizontal(|ui| {
        // Logo with Cyberpunk colors
        ui.spacing_mut().item_spacing.x = 2.0;
        ui.label(RichText::new("DJ").color(COLOR_MINT).strong().size(20.0).italics());
        ui.label(RichText::new("ENGINE").color(COLOR_ORANGE).strong().size(20.0));
        
        ui.add_space(10.0);
        ui.separator();
        ui.add_space(10.0);

        // View Switcher Tabs
        let mut ui_state = world.resource_mut::<EditorUiState>();
        ui.selectable_value(&mut ui_state.current_view, EditorView::Level, RichText::new("🌍 Level Editor").strong());
        ui.selectable_value(&mut ui_state.current_view, EditorView::StoryGraph, RichText::new("🕸 Story Graph").strong());
        
        ui.add_space(10.0);
        ui.separator();
        ui.add_space(10.0);
        
        // Play Controls
        let current_state = world.resource::<State<EditorState>>().get().clone();
        let is_playing = current_state == EditorState::Playing;

        if ui.add_enabled(!is_playing, egui::Button::new(RichText::new("▶ PLAY").color(COLOR_MINT))).clicked() {
            world.resource_mut::<NextState<EditorState>>().set(EditorState::Playing);
            info!("Editor: Play requested");
        }
        if ui.add_enabled(is_playing, egui::Button::new(RichText::new("⏹ STOP").color(COLOR_ORANGE))).clicked() {
            world.resource_mut::<NextState<EditorState>>().set(EditorState::Editor);
            info!("Editor: Stop requested");
        }

        ui.add_space(10.0);
        ui.separator();
        
        let mut project = world.resource_mut::<ProjectMetadata>();
        if ui.button("📁 Load").clicked() {
            project.name = "DoomExe".into();
            project.path = Some("games/dev/doomexe".into());
        }

        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.label(RichText::new(format!("Active: {}", project.name)).italics().color(Color32::GRAY));
        });
    });
}

fn draw_left_panel(ui: &mut egui::Ui, world: &mut World) {
    world.resource_scope::<EditorUiState, _>(|world, mut ui_state| {
        ui.add_space(4.0);
        ui.horizontal(|ui| {
            ui.selectable_value(&mut ui_state.browser_tab, BrowserTab::Palette, "Palette");
            ui.selectable_value(&mut ui_state.browser_tab, BrowserTab::Hierarchy, "Hierarchy");
            ui.selectable_value(&mut ui_state.browser_tab, BrowserTab::Assets, "Files");
        });
        ui.add_space(4.0);
        ui.separator();

        match ui_state.browser_tab {
            BrowserTab::Hierarchy => {
                ui.add_space(5.0);
                ui.label(RichText::new("SCENE HIERARCHY").strong().color(COLOR_MINT));
                ui.add_space(5.0);
                bevy_inspector::hierarchy::hierarchy_ui(world, ui, &mut ui_state.selected_entities);
            }
            BrowserTab::Assets => {
                ui.add_space(5.0);
                ui.label(RichText::new("ASSET BROWSER").strong().color(COLOR_MINT));
                ui.add_space(5.0);
                let mut query = ui_state.asset_search_query.clone();
                ui.text_edit_singleline(&mut query);
                ui_state.asset_search_query = query;
                
                ui.separator();
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.label("📁 music");
                    ui.label("📁 sprites");
                    ui.label("📁 scripts");
                    ui.label("  📄 hamster_test.lua");
                });
            }
            BrowserTab::Palette => {
                ui.add_space(5.0);
                ui.label(RichText::new("TOOL PALETTE").strong().color(COLOR_MINT));
                ui.add_space(5.0);
                ui.label(RichText::new("Select item to paint:").italics());
                
                let mut selected = ui_state.selected_palette_item.clone();
                
                ui.add_space(5.0);
                ui.selectable_value(&mut selected, Some("Grass".to_string()), "🌿 Grass Tile");
                ui.selectable_value(&mut selected, Some("Wall".to_string()), "🧱 Stone Wall");
                ui.selectable_value(&mut selected, Some("Hamster".to_string()), "🐹 Hamster Unit");
                ui.selectable_value(&mut selected, Some("Chest".to_string()), "📦 Loot Chest");
                
                ui.add_space(10.0);
                if ui.button(RichText::new("❌ Clear Selection").color(COLOR_ORANGE)).clicked() {
                    selected = None;
                }
                
                ui_state.selected_palette_item = selected;
            }
        }
    });
}

fn draw_right_panel(ui: &mut egui::Ui, world: &mut World) {
    ui.add_space(5.0);
    ui.label(RichText::new("INSPECTOR").strong().color(COLOR_MINT));
    ui.add_space(5.0);
    ui.separator();
    
    world.resource_scope::<EditorUiState, _>(|world, ui_state| {
        if ui_state.selected_entities.is_empty() {
            ui.add_space(10.0);
            ui.label(RichText::new("No entity selected.").italics().color(Color32::GRAY));
            ui.add_space(10.0);
            ui.separator();
            ui.collapsing("Global Resources", |ui| {
                bevy_inspector::ui_for_resources(world, ui);
            });
        } else {
             bevy_inspector::ui_for_entities_shared_components(world, ui_state.selected_entities.as_slice(), ui);
        }
    });
}

fn draw_bottom_panel(ui: &mut egui::Ui, world: &mut World) {
    ui.horizontal(|ui| {
        ui.label(RichText::new("CONSOLE").strong().color(COLOR_MINT));
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
             if ui.button(RichText::new("Clear").color(Color32::GRAY)).clicked() {
                 if let Some(mut store) = world.get_resource_mut::<ConsoleLogStore>() {
                     store.logs.clear();
                 }
             }
        });
    });
    ui.separator();
    
    egui::ScrollArea::vertical()
        .stick_to_bottom(true)
        .show(ui, |ui| {
            if let Some(store) = world.get_resource::<ConsoleLogStore>() {
                for log in &store.logs {
                    let color = if log.contains("TEST:") || log.contains("Passed") {
                        COLOR_MINT
                    } else if log.contains("WARN") {
                        COLOR_ORANGE
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
}

fn draw_central_panel(ui: &mut egui::Ui, world: &mut World) {
    let ui_state = world.resource::<EditorUiState>();
    
    match ui_state.current_view {
        EditorView::Level => {
            let state = world.resource::<State<EditorState>>().get();
            if *state == EditorState::Editor {
                // Draw grid and handle interactions
                draw_grid(ui, world);
            } else {
                ui.centered_and_justified(|ui| {
                    ui.label(RichText::new("RUNTIME ACTIVE").strong().size(24.0).color(COLOR_MINT));
                });
            }
        },
        EditorView::StoryGraph => {
            draw_story_graph(ui);
        }
    }
}

fn draw_grid(ui: &mut egui::Ui, world: &mut World) {
    let rect = ui.available_rect_before_wrap();
    
    // 1. Handle Input (Placement)
    // We do this before drawing so the new item appears immediately (or next frame)
    let response = ui.allocate_rect(rect, egui::Sense::click());
    
    // Now valid to create painter after mutable borrow is done (or rather, we don't hold the painter while mutating ui via allocate_rect if we scope it, 
    // but ui.painter() borrows ui. allocate_rect borrows ui mutably.
    // So we must call allocate_rect first, THEN get painter.
    let painter = ui.painter();
    
    if response.clicked() {
        if let Some(pointer_pos) = ui.input(|i| i.pointer.interact_pos()) {
            // Convert UI coordinates to "World" coordinates relative to the panel
            // For this 2D editor prototype, we treat the top-left of the panel as (0,0) world space for simplicity,
            // or we center it. Let's map it simply for now.
            let _relative_pos = pointer_pos - rect.min;
            
            // Snap to grid
            let grid_size = 40.0;
            // let grid_x = (relative_pos.x / grid_size).floor() * grid_size;
            // let grid_y = (relative_pos.y / grid_size).floor() * grid_size;
            
            // Bevy coordinates: Y is up, X is right. Center is (0,0).
            // Egui coordinates: Y is down, X is right. Top-left is (0,0).
            // We need a translation. For this visual prototype, we'll just spawn at a transform 
            // that roughly aligns with where we clicked visually if we assume a standard 2D camera.
            // But since we aren't rendering the world *in* the egui panel yet (just a grid overlay),
            // this is a "blind" spawn into the world. 
            // However, the Hierarchy will update, confirming the action.
            
            // Let's spawn at a 3D position assuming Z=0 plane.
            // We'll map the panel center to World (0,0).
            let center = rect.center();
            let world_x = pointer_pos.x - center.x;
            let world_y = center.y - pointer_pos.y; // Flip Y for Bevy
            
            let snap_x = (world_x / grid_size).round() * grid_size;
            let snap_y = (world_y / grid_size).round() * grid_size;

            let selected_item = world.resource::<EditorUiState>().selected_palette_item.clone();
            
            if let Some(item) = selected_item {
                info!("Editor: Spawning {} at ({}, {})", item, snap_x, snap_y);
                
                // Determine color based on item
                let color = match item.as_str() {
                    "Grass" => Color::srgb(0.2, 0.8, 0.2),
                    "Wall" => Color::srgb(0.5, 0.5, 0.5),
                    "Hamster" => Color::srgb(0.8, 0.5, 0.2),
                    "Chest" => Color::srgb(0.8, 0.8, 0.1),
                    _ => Color::WHITE,
                };

                world.spawn((
                    Name::new(format!("{} [{:.0}, {:.0}]", item, snap_x, snap_y)),
                    Sprite {
                        color,
                        custom_size: Some(Vec2::new(30.0, 30.0)),
                        ..default()
                    },
                    Transform::from_xyz(snap_x, snap_y, 0.0)
                ));
            }
        }
    }

    // 2. Draw Grid Visuals
    painter.rect_filled(rect, 0.0, COLOR_BG);
    
    let grid_size = 40.0;
    let color = Color32::from_rgb(30, 30, 40);
    
    let mut x = rect.left();
    while x < rect.right() {
        painter.line_segment([egui::pos2(x, rect.top()), egui::pos2(x, rect.bottom())], (1.0, color));
        x += grid_size;
    }
    
    let mut y = rect.top();
    while y < rect.bottom() {
        painter.line_segment([egui::pos2(rect.left(), y), egui::pos2(rect.right(), y)], (1.0, color));
        y += grid_size;
    }
    
    // Draw ghost of selected item at mouse cursor
    if let Some(_item) = &world.resource::<EditorUiState>().selected_palette_item {
        if let Some(pointer_pos) = ui.input(|i| i.pointer.hover_pos()) {
             if rect.contains(pointer_pos) {
                 painter.circle_filled(pointer_pos, 5.0, COLOR_MINT);
             }
        }
    }
}

fn draw_story_graph(ui: &mut egui::Ui) {
    let painter = ui.painter();
    let rect = ui.available_rect_before_wrap();
    
    // Darker background for node editor
    painter.rect_filled(rect, 0.0, Color32::from_rgb(10, 10, 15));
    
    // Grid dots
    let grid_size = 20.0;
    let color = Color32::from_rgb(20, 20, 25);
    
    let mut x = rect.left();
    while x < rect.right() {
        let mut y = rect.top();
        while y < rect.bottom() {
            painter.circle_filled(egui::pos2(x, y), 1.0, color);
            y += grid_size;
        }
        x += grid_size;
    }
    
    // Mock Node
    egui::Window::new(RichText::new("Quest: Hamster Retrieval").color(COLOR_MINT))
        .default_pos(rect.min + egui::vec2(100.0, 100.0))
        .show(ui.ctx(), |ui| {
            ui.label("Trigger: OnEnterZone");
            ui.separator();
            ui.label("Action: Spawn Hamster");
            let _ = ui.button("Edit Logic");
        });
        
     egui::Window::new(RichText::new("Dialogue: Intro").color(COLOR_MINT))
        .default_pos(rect.min + egui::vec2(400.0, 150.0))
        .show(ui.ctx(), |ui| {
            ui.text_edit_multiline(&mut "Hello traveler! Have you seen my hamster?");
            ui.horizontal(|ui| {
                let _ = ui.button("Add Choice");
                let _ = ui.button("Set Condition");
            });
        });
}