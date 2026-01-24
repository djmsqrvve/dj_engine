use bevy::prelude::*;
use bevy_egui::egui::{self, RichText, Color32};
use super::state::*;

pub mod panels;
pub mod views;
pub mod campaign;
pub mod phases_view;

pub fn editor_ui_system(world: &mut World) {
    let ctx = world
        .query_filtered::<&mut bevy_egui::EguiContext, With<bevy::window::PrimaryWindow>>()
        .get_single_mut(world)
        .map(|mut c| c.get_mut().clone())
        .expect("Missing primary window EguiContext");
        
    egui::TopBottomPanel::top("top_panel").show(&ctx, |ui| {
        panels::draw_top_menu(ui, world);
    });

    // Bottom Console Panel (always docked)
    egui::TopBottomPanel::bottom("console_panel")
        .default_height(150.0)
        .resizable(true)
        .show(&ctx, |ui| {
            panels::draw_console_panel(ui, world);
        });

    egui::SidePanel::left("left_panel")
        .default_width(250.0)
        .show(&ctx, |ui| {
            panels::draw_left_panel(ui, world);
        });

    egui::SidePanel::right("right_panel")
        .default_width(300.0)
        .show(&ctx, |ui| {
            panels::draw_right_panel(ui, world);
        });

    let current_state = world.resource::<State<EditorState>>().get();
    let central_frame = if *current_state == EditorState::Playing {
        egui::Frame::none()
    } else {
        egui::Frame::central_panel(&ctx.style())
    };

    egui::CentralPanel::default().frame(central_frame).show(&ctx, |ui| {
        // Capture the actual rect for the camera viewport!
        let rect = ui.max_rect();
        if let Some(mut viewport) = world.get_resource_mut::<crate::rendering::ViewportRect>() {
             viewport.0 = Some(bevy::math::Rect::new(rect.min.x, rect.min.y, rect.max.x, rect.max.y));
        }
        
        draw_central_panel(ui, world);
    });
}

fn draw_central_panel(ui: &mut egui::Ui, world: &mut World) {
    // Determine which view to show
    let current_view = {
        let ui_state = world.resource::<EditorUiState>();
        if ui_state.global_view == EditorView::Core {
            EditorView::Core
        } else if let Some(branch) = ui_state.current_branch() {
            branch.active_view.clone()
        } else {
            EditorView::default()
        }
    };
    
    match current_view {
        EditorView::Core => {
            views::draw_core_dashboard(ui, world);
        }

        EditorView::MapEditor | EditorView::ScenarioEditor => {
            let state = world.resource::<State<EditorState>>().get();
            if *state == EditorState::Editor {
                // Draw grid and handle interactions
                views::draw_grid(ui, world);
            } else {
                 // Subtle overlay indicator
                 ui.with_layout(egui::Layout::top_down(egui::Align::Min), |ui| {
                    ui.label(RichText::new("● LIVE").color(Color32::RED).small());
                 });
            }
        },
        EditorView::StoryGraph => {
            views::draw_story_graph(ui, world);
        }
        EditorView::Campaign => {
            let mut state = world.resource_mut::<campaign::CampaignEditorState>();
            campaign::draw_campaign_editor(ui, &mut state);
        }
        EditorView::Controls => {
            views::draw_controls_view(ui, world);
        }
        EditorView::Settings => {
            views::draw_settings_view(ui, world);
        }
        EditorView::Phases => {
            phases_view::draw_phases_view(ui, world);
        }
        EditorView::Play => {
            let current_state = world.resource::<State<EditorState>>().get().clone();
            
            if current_state == EditorState::Editor {
                ui.centered_and_justified(|ui| {
                    if ui.button(RichText::new("▶ START GAME").size(30.0).color(COLOR_PRIMARY)).clicked() {
                        // Launch logic
                        world.resource_scope::<ActiveStoryGraph, _>(|world, graph| {
                             if let Some(mut executor) = world.get_resource_mut::<crate::story_graph::GraphExecutor>() {
                                 executor.load_from_data(&graph.0);
                                 info!("Editor: Loaded Story Graph into Executor");
                             }
                        });
                        world.resource_mut::<NextState<EditorState>>().set(EditorState::Playing);
                    }
                });
            } else {
                // In Playing state
                // Draw a small overlay for Stop
                ui.with_layout(egui::Layout::top_down(egui::Align::Min), |ui| {
                    ui.horizontal(|ui| {
                        ui.label(RichText::new("● LIVE").color(Color32::RED).small());
                        if ui.button(RichText::new("⏹ STOP").color(COLOR_SECONDARY)).clicked() {
                            world.resource_mut::<NextState<EditorState>>().set(EditorState::Editor);
                        }
                    });
                });
                // The rest is transparent to show the game
            }
        }
    }
}
