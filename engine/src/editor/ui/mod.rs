use super::state::*;
use bevy::prelude::*;
use bevy_egui::egui::{self, Color32, RichText};

pub mod campaign;
pub mod feature_grid;
pub mod panels;
pub mod phases_view;
pub mod timeline;
pub mod views;
pub mod workspace;

pub struct EditorTabViewer<'a> {
    pub world: &'a mut World,
}

impl<'a> egui_dock::TabViewer for EditorTabViewer<'a> {
    type Tab = EditorView;

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        match tab {
            EditorView::Core => "📦 Core".into(),
            EditorView::FeatureGrid => "🔷 Feature Grid".into(),
            EditorView::Timeline => "🎹 Timeline".into(),
            EditorView::MapEditor => "🗺 Map".into(),
            EditorView::ScenarioEditor => "🎭 Scenario".into(),
            EditorView::StoryGraph => "📽 Story Graph".into(),
            EditorView::Campaign => "📅 Campaign".into(),
            EditorView::Play => "🎮 Play".into(),
            EditorView::Settings => "⚙ Settings".into(),
            EditorView::Phases => "Phases".into(),
            EditorView::Controls => "Controls".into(),
            EditorView::Hierarchy => "Hierarchy".into(),
            EditorView::Palette => "Palette".into(),
            EditorView::Assets => "Assets".into(),
            EditorView::Inspector => "Inspector".into(),
            EditorView::Console => "💻 Console".into(),
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        match tab {
            EditorView::Core => views::draw_core_dashboard(ui, self.world),
            EditorView::FeatureGrid => feature_grid::draw_feature_grid(ui, self.world),
            EditorView::Timeline => timeline::draw_timeline_view(ui, self.world),
            EditorView::MapEditor => views::draw_grid(ui, self.world), // Assuming map editor needs grid
            EditorView::ScenarioEditor => views::draw_grid(ui, self.world),
            EditorView::StoryGraph => views::draw_story_graph(ui, self.world),
            EditorView::Campaign => {
                // Needs mutable borrow of CampaignEditorState
                // We can fetch it from world
                if let Some(mut state) = self
                    .world
                    .get_resource_mut::<campaign::CampaignEditorState>()
                {
                    campaign::draw_campaign_editor(ui, &mut state);
                }
            }
            EditorView::Settings => views::draw_settings_view(ui, self.world),
            EditorView::Controls => views::draw_controls_view(ui, self.world),
            EditorView::Phases => phases_view::draw_phases_view(ui, self.world),
            EditorView::Play => {
                // Play view logic
                ui.centered_and_justified(|ui| {
                    if ui
                        .button(
                            RichText::new("▶ START GAME")
                                .size(30.0)
                                .color(COLOR_PRIMARY),
                        )
                        .clicked()
                    {
                        self.world
                            .resource_scope::<ActiveStoryGraph, _>(|world, graph| {
                                world.resource_scope::<crate::story_graph::StoryGraphLibrary, _>(|world, mut library| {
                                    if let Some(mut executor) =
                                        world.get_resource_mut::<crate::story_graph::GraphExecutor>()
                                    {
                                        executor.load_from_data(&graph.0, &mut library);
                                        info!("Editor: Loaded Story Graph into Executor");
                                    }
                                });
                            });
                        self.world
                            .resource_mut::<NextState<EditorState>>()
                            .set(EditorState::Playing);
                    }
                });
            }
            EditorView::Hierarchy => {
                // Hierarchy Logic
                use bevy_inspector_egui::bevy_inspector;
                // We need EditorUiState to get selected_entities.
                // Use resource_scope to avoid double borrow of world
                if self.world.contains_resource::<EditorUiState>() {
                    self.world.resource_scope::<EditorUiState, _>(|world, mut ui_state| {
                        egui::ScrollArea::vertical().show(ui, |ui| {
                            bevy_inspector::hierarchy::hierarchy_ui(
                                world,
                                ui,
                                &mut ui_state.selected_entities,
                            );
                        });
                    });
                }
            }
            EditorView::Palette => {
                // Palette Logic (Stub from panels.rs)
                panels::draw_palette_content(ui);
            }
            EditorView::Assets => {
                ui.label("Asset Browser");
            }
            EditorView::Inspector => {
                // Inspector Logic
                // We need to call panels::draw_inspector_content or similar.
                // Since draw_right_panel logic is complex, I should refactor panels.rs to expose the inner content
                // OR I can just access panels::draw_right_panel_content if I verify/create it.
                // For now, I will assume panels::draw_right_panel handles logic using World, but I need to adapt it.
                // panels::draw_right_panel uses `ui` and `world`. It should work as is IF I strip the SidePanel wrapper in it?
                // No, panels::draw_right_panel draws the HEADER ("INSPECTOR").
                panels::draw_right_panel(ui, self.world);
            }
            EditorView::Console => {
                panels::draw_console_panel(ui, self.world);
            }
        }
    }

    fn closeable(&mut self, tab: &mut Self::Tab) -> bool {
        match tab {
            EditorView::Core
            | EditorView::FeatureGrid
            | EditorView::Timeline
            | EditorView::MapEditor
            | EditorView::ScenarioEditor
            | EditorView::StoryGraph
            | EditorView::Campaign
            | EditorView::Play
            | EditorView::Settings
            | EditorView::Controls
            | EditorView::Phases => false,
            _ => true,
        }
    }
}

pub fn editor_ui_system(world: &mut World) {
    let ctx = world
        .query_filtered::<&mut bevy_egui::EguiContext, With<bevy::window::PrimaryWindow>>()
        .get_single_mut(world)
        .map(|mut c| c.get_mut().clone())
        .expect("Missing primary window EguiContext");

    // Top Menu
    egui::TopBottomPanel::top("top_panel").show(&ctx, |ui| {
        panels::draw_top_menu(ui, world);
    });

    // Workspace Canvas (Rest of the screen)
    egui::CentralPanel::default()
        .frame(egui::Frame::none().fill(Color32::TRANSPARENT))
        .show(&ctx, |ui| {
            workspace::draw_workspace_canvas(ui, world);
        });
}
