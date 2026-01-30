use crate::data::scene::{Entity, EntityType, Scene};
use crate::data::story::{RequiredEntity, SceneValidationError, StoryGraphData};
use bevy::prelude::*;
use bevy_egui::egui;

/// Resource for managing editor validation state.
#[derive(Resource, Default)]
pub struct ValidationState {
    pub errors: Vec<SceneValidationError>,
    pub last_validation_time: f64,
}

/// Find the required entity type for a given entity ID in the graph.
fn find_required_entity_type<'a>(graph: &'a StoryGraphData, entity_id: &str) -> Option<&'a RequiredEntity> {
    graph.nodes.iter().find_map(|node| {
        node.required_entities.iter().find(|r| r.entity_id == entity_id)
    })
}

/// Panel for displaying validation errors and missing items.
pub fn draw_validation_panel(
    ui: &mut egui::Ui,
    validation_state: &mut ValidationState,
    active_graph: Option<&StoryGraphData>,
    active_scene: Option<&mut Scene>,
) {
    ui.heading("Validation & Missing Items");
    ui.separator();

    if active_graph.is_none() || active_scene.is_none() {
        ui.label("Open a Scene and a Story Graph to validate.");
        return;
    }

    let graph = active_graph.unwrap();
    let scene = active_scene.unwrap();

    validation_state.errors = graph.validate_against_scene(scene);

    if validation_state.errors.is_empty() {
        ui.label(egui::RichText::new("✓ All checks passed").color(egui::Color32::GREEN));
        return;
    }

    ui.label(
        egui::RichText::new(format!("⚠ Found {} issues", validation_state.errors.len()))
            .color(egui::Color32::YELLOW),
    );
    ui.separator();

    egui::ScrollArea::vertical().show(ui, |ui| {
        let mut fixes_to_apply = Vec::new();

        for (index, error) in validation_state.errors.iter().enumerate() {
            ui.group(|ui| {
                match error {
                    SceneValidationError::MissingRequiredEntity { node_id, entity_id } => {
                        ui.horizontal(|ui| {
                            ui.label(
                                egui::RichText::new("Missing Entity")
                                    .strong()
                                    .color(egui::Color32::RED),
                            );
                            ui.label(format!(
                                "Node '{}' requires entity '{}'",
                                node_id, entity_id
                            ));
                        });

                        if ui.button(format!("Fix: Spawn {}", entity_id)).clicked() {
                            fixes_to_apply.push((index, FixAction::SpawnEntity(entity_id.clone())));
                        }
                    }
                    SceneValidationError::WrongEntityType {
                        node_id,
                        entity_id,
                        expected,
                        found,
                    } => {
                        ui.horizontal(|ui| {
                            ui.label(
                                egui::RichText::new("Type Mismatch")
                                    .strong()
                                    .color(egui::Color32::ORANGE),
                            );
                            ui.label(format!(
                                "Node '{}' expects '{}' to be {:?}, found {:?}",
                                node_id, entity_id, expected, found
                            ));
                        });

                        if ui.button("Fix: Update Type").clicked() {
                            fixes_to_apply.push((index, FixAction::UpdateType(entity_id.clone())));
                        }
                    }
                }
            });
        }

        apply_fixes(fixes_to_apply, graph, scene);
    });
}

/// Actions that can be applied to fix validation errors.
enum FixAction {
    SpawnEntity(String),
    UpdateType(String),
}

/// Apply collected fixes to the scene.
fn apply_fixes(fixes: Vec<(usize, FixAction)>, graph: &StoryGraphData, scene: &mut Scene) {
    for (_, action) in fixes {
        match action {
            FixAction::SpawnEntity(id) => {
                let mut new_entity = Entity::new(id.clone(), id.clone());
                new_entity.entity_type = find_required_entity_type(graph, &id)
                    .and_then(|req| req.entity_type)
                    .unwrap_or(EntityType::Npc);
                scene.add_entity(new_entity);
            }
            FixAction::UpdateType(id) => {
                let Some(entity) = scene.find_entity_mut(&id) else { continue };
                let Some(req) = find_required_entity_type(graph, &id) else { continue };
                let Some(etype) = req.entity_type else { continue };
                entity.entity_type = etype;
            }
        }
    }
}
