use bevy::prelude::*;
use bevy_egui::egui;
use crate::data::campaign::{CampaignData, CampaignNodeType};

#[derive(Resource, Default)]
pub struct CampaignEditorState {
    pub active_campaign: CampaignData,
    pub selection: Option<String>,
    pub pan: Vec2,
    pub zoom: f32,
}

pub fn draw_campaign_editor(
    ui: &mut egui::Ui,
    state: &mut CampaignEditorState,
) {
    ui.heading("Campaign Board");
    ui.separator();

    let rect = ui.available_rect_before_wrap();
    let response = ui.allocate_rect(rect, egui::Sense::drag());
    let painter = ui.painter();

    // Handle Pan
    if response.dragged() {
        let delta = response.drag_delta();
        state.pan += Vec2::new(delta.x, delta.y);
    }

    // Draw Nodes
    for node in &state.active_campaign.nodes {
        let node_pos = egui::pos2(
            rect.min.x + node.position.x + state.pan.x,
            rect.min.y + node.position.y + state.pan.y
        );

        // ACT Container
        if node.node_type == CampaignNodeType::Act {
            let act_rect = egui::Rect::from_min_size(node_pos, egui::vec2(300.0, 200.0));
            painter.rect_stroke(
                act_rect, 
                5.0, 
                egui::Stroke::new(2.0, egui::Color32::from_rgb(100, 100, 150))
            );
            painter.text(
                node_pos + egui::vec2(10.0, 10.0),
                egui::Align2::LEFT_TOP,
                &node.name,
                egui::FontId::proportional(20.0),
                egui::Color32::WHITE,
            );
        } 
        // Normal Node
        else {
            let color = match node.node_type {
                CampaignNodeType::Start => egui::Color32::GREEN,
                CampaignNodeType::End => egui::Color32::RED,
                CampaignNodeType::StoryGraph => egui::Color32::LIGHT_BLUE,
                CampaignNodeType::Scene => egui::Color32::GOLD,
                CampaignNodeType::Act => egui::Color32::TRANSPARENT, // Handled above
            };

            painter.circle_filled(node_pos, 20.0, color);
            painter.text(
                node_pos,
                egui::Align2::CENTER_CENTER,
                &node.name,
                egui::FontId::proportional(14.0),
                egui::Color32::BLACK,
            );
        }

        // Draw Connections arrow
        for next_id in &node.next_node_ids {
            if let Some(target) = state.active_campaign.nodes.iter().find(|n| n.id == *next_id) {
                 let target_pos = egui::pos2(
                    rect.min.x + target.position.x + state.pan.x,
                    rect.min.y + target.position.y + state.pan.y
                );
                painter.arrow(
                    node_pos, 
                    target_pos - node_pos, 
                    egui::Stroke::new(2.0, egui::Color32::GRAY)
                );
            }
        }
    }

    ui.label("Drag to pan. (Placeholder UI)");
}
