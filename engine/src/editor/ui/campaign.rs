use crate::data::campaign::{CampaignData, CampaignNodeType};
use bevy::prelude::*;
use bevy_egui::egui;

#[derive(Resource, Default)]
pub struct CampaignEditorState {
    pub active_campaign: CampaignData,
    pub pan: Vec2,
}

pub fn draw_campaign_editor(ui: &mut egui::Ui, state: &mut CampaignEditorState) {
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
            rect.min.y + node.position.y + state.pan.y,
        );

        // ACT Container
        if node.node_type == CampaignNodeType::Act {
            let act_rect = egui::Rect::from_min_size(node_pos, egui::vec2(320.0, 240.0));
            // Multi-layered visual for "Premium" feel
            painter.rect_filled(
                act_rect,
                5.0,
                egui::Color32::from_rgba_unmultiplied(100, 100, 200, 20),
            );
            painter.rect_stroke(
                act_rect,
                5.0,
                egui::Stroke::new(3.0, egui::Color32::from_rgb(0, 255, 204)), // COLOR_PRIMARY (Mint)
                egui::StrokeKind::Inside,
            );
            painter.text(
                node_pos + egui::vec2(15.0, 15.0),
                egui::Align2::LEFT_TOP,
                node.name.to_uppercase(),
                egui::FontId::proportional(22.0),
                egui::Color32::from_rgb(0, 255, 204),
            );
        }
        // Normal Node
        else {
            let (fill, stroke_color) = match node.node_type {
                CampaignNodeType::Start => {
                    (egui::Color32::from_rgb(0, 100, 0), egui::Color32::GREEN)
                }
                CampaignNodeType::End => (egui::Color32::from_rgb(100, 0, 0), egui::Color32::RED),
                CampaignNodeType::StoryGraph => (
                    egui::Color32::from_rgb(0, 50, 100),
                    egui::Color32::LIGHT_BLUE,
                ),
                CampaignNodeType::Scene => {
                    (egui::Color32::from_rgb(100, 80, 0), egui::Color32::GOLD)
                }
                CampaignNodeType::Act => (egui::Color32::TRANSPARENT, egui::Color32::TRANSPARENT),
            };

            // Glow effect for nodes
            painter.circle_filled(node_pos, 25.0, stroke_color.linear_multiply(0.2));
            painter.circle_filled(node_pos, 22.0, fill);
            painter.circle_stroke(node_pos, 22.0, egui::Stroke::new(2.0, stroke_color));

            painter.text(
                node_pos,
                egui::Align2::CENTER_CENTER,
                &node.name,
                egui::FontId::proportional(12.0),
                egui::Color32::WHITE,
            );
        }

        // Draw Connections arrow
        for next_id in &node.next_node_ids {
            if let Some(target) = state
                .active_campaign
                .nodes
                .iter()
                .find(|n| n.id == *next_id)
            {
                let target_pos = egui::pos2(
                    rect.min.x + target.position.x + state.pan.x,
                    rect.min.y + target.position.y + state.pan.y,
                );
                painter.arrow(
                    node_pos,
                    target_pos - node_pos,
                    egui::Stroke::new(2.0, egui::Color32::GRAY),
                );
            }
        }
    }

    if state.active_campaign.nodes.is_empty() {
        ui.centered_and_justified(|ui| {
            ui.label(
                bevy_egui::egui::RichText::new("Empty Campaign. Add nodes via Sidebar.")
                    .italics()
                    .color(bevy_egui::egui::Color32::GRAY),
            );
        });
    }
}
