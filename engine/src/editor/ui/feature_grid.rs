//! Feature Grid - Visual ecosystem toggle system

use bevy::prelude::*;
use bevy_egui::egui::{self, Color32, Pos2, RichText, Stroke};

use super::super::state::*;

/// Draw a hexagon at the given center
fn draw_hexagon(painter: &egui::Painter, center: Pos2, size: f32, fill: Color32, stroke: Stroke) {
    let mut points = Vec::with_capacity(6);
    for i in 0..6 {
        let angle = (i as f32 / 6.0) * std::f32::consts::TAU - std::f32::consts::FRAC_PI_6;
        points.push(center + egui::vec2(angle.cos(), angle.sin()) * size);
    }
    painter.add(egui::Shape::convex_polygon(points, fill, stroke));
}

pub fn draw_feature_grid(ui: &mut egui::Ui, world: &mut World) {
    let rect = ui.available_rect_before_wrap();
    let center = rect.center();

    // Background
    ui.painter()
        .rect_filled(rect, 0.0, Color32::from_rgb(8, 8, 12));

    // VISUAL DEMO banner
    let banner_rect = egui::Rect::from_min_size(rect.left_top(), egui::vec2(rect.width(), 24.0));
    ui.painter()
        .rect_filled(banner_rect, 0.0, Color32::from_rgb(80, 60, 20));
    ui.painter().text(
        banner_rect.center(),
        egui::Align2::CENTER_CENTER,
        "⚠ VISUAL DEMO — Feature toggling is for visualization only",
        egui::FontId::proportional(12.0),
        Color32::from_rgb(255, 200, 100),
    );

    // Draw subtle hex grid pattern in background
    let grid_color = Color32::from_rgba_unmultiplied(255, 255, 255, 10);
    for x in (-10..10).map(|i| i as f32 * 60.0) {
        for y in (-10..10).map(|i| i as f32 * 52.0) {
            let offset = if (y / 52.0) as i32 % 2 == 0 {
                0.0
            } else {
                30.0
            };
            let pos = center + egui::vec2(x + offset, y);
            if rect.contains(pos) {
                draw_hexagon(
                    ui.painter(),
                    pos,
                    25.0,
                    Color32::TRANSPARENT,
                    Stroke::new(0.5, grid_color),
                );
            }
        }
    }

    // Draw CORE hub at center
    draw_hexagon(
        ui.painter(),
        center,
        50.0,
        Color32::from_rgb(20, 20, 30),
        Stroke::new(3.0, COLOR_PRIMARY),
    );
    ui.painter().text(
        center,
        egui::Align2::CENTER_CENTER,
        "CORE",
        egui::FontId::proportional(18.0),
        COLOR_PRIMARY,
    );

    // Get state
    let (active_branch_idx, node_overrides) = {
        let ui_state = world.resource::<EditorUiState>();
        let branch = &ui_state.active_branches[ui_state.active_branch_idx];
        (ui_state.active_branch_idx, branch.node_overrides.clone())
    };

    let feature_grid = world.resource::<FeatureGrid>();
    let mut toggle_node: Option<String> = None;

    // Draw ecosystems
    for ecosystem in &feature_grid.ecosystems {
        let eco_center = center + egui::vec2(ecosystem.position.x, ecosystem.position.y);

        // Connection line from CORE
        ui.painter().line_segment(
            [center, eco_center],
            Stroke::new(2.0, ecosystem.color.linear_multiply(0.4)),
        );

        // Ecosystem label hexagon
        draw_hexagon(
            ui.painter(),
            eco_center,
            40.0,
            ecosystem.color.linear_multiply(0.2),
            Stroke::new(2.0, ecosystem.color),
        );
        ui.painter().text(
            eco_center,
            egui::Align2::CENTER_CENTER,
            &ecosystem.icon,
            egui::FontId::proportional(20.0),
            ecosystem.color,
        );
        ui.painter().text(
            eco_center + egui::vec2(0.0, 50.0),
            egui::Align2::CENTER_CENTER,
            &ecosystem.name,
            egui::FontId::proportional(14.0),
            Color32::WHITE,
        );

        // Draw feature nodes around ecosystem
        for (i, node) in ecosystem.nodes.iter().enumerate() {
            let angle = (i as f32 / ecosystem.nodes.len() as f32) * std::f32::consts::TAU
                - std::f32::consts::FRAC_PI_2;
            let node_pos = eco_center + egui::vec2(angle.cos(), angle.sin()) * 80.0;

            let is_enabled = node_overrides.get(&node.id).copied().unwrap_or(true);

            let fill = if is_enabled {
                ecosystem.color.linear_multiply(0.3)
            } else {
                Color32::from_rgb(40, 40, 40)
            };
            let stroke_color = if is_enabled {
                ecosystem.color
            } else {
                Color32::DARK_GRAY
            };

            // Connection to ecosystem center
            ui.painter().line_segment(
                [eco_center, node_pos],
                Stroke::new(1.0, stroke_color.linear_multiply(0.5)),
            );

            // Node hexagon
            draw_hexagon(
                ui.painter(),
                node_pos,
                28.0,
                fill,
                Stroke::new(1.5, stroke_color),
            );

            // Icon and name
            ui.painter().text(
                node_pos - egui::vec2(0.0, 5.0),
                egui::Align2::CENTER_CENTER,
                &node.icon,
                egui::FontId::proportional(16.0),
                Color32::WHITE,
            );
            ui.painter().text(
                node_pos + egui::vec2(0.0, 35.0),
                egui::Align2::CENTER_CENTER,
                &node.name,
                egui::FontId::proportional(10.0),
                if is_enabled {
                    Color32::WHITE
                } else {
                    Color32::DARK_GRAY
                },
            );

            // Status indicator
            let status_pos = node_pos + egui::vec2(20.0, -20.0);
            let status_color = if is_enabled {
                Color32::GREEN
            } else {
                Color32::RED
            };
            ui.painter().circle_filled(status_pos, 5.0, status_color);

            // Interaction
            let node_rect = egui::Rect::from_center_size(node_pos, egui::vec2(56.0, 56.0));
            let response = ui.allocate_rect(node_rect, egui::Sense::click());

            if response.clicked() {
                toggle_node = Some(node.id.clone());
            }

            response.on_hover_ui(|ui| {
                ui.label(RichText::new(&node.name).strong());
                ui.label(&node.description);
                ui.label(if is_enabled {
                    "✅ Enabled"
                } else {
                    "❌ Disabled"
                });
            });
        }
    }

    // Apply toggle
    if let Some(node_id) = toggle_node {
        if let Some(mut ui_state) = world.get_resource_mut::<EditorUiState>() {
            if let Some(branch) = ui_state.active_branches.get_mut(active_branch_idx) {
                let current = branch.node_overrides.get(&node_id).copied().unwrap_or(true);
                branch.node_overrides.insert(node_id, !current);
            }
        }
    }
}
