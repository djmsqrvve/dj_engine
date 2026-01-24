use crate::core::phases::{GamePhase, PhaseManager, TaskStatus};
use crate::editor::state::COLOR_PRIMARY;
use bevy::prelude::*;
use bevy_egui::egui::{self, Color32, RichText};

pub fn draw_phases_view(ui: &mut egui::Ui, world: &mut World) {
    ui.heading(RichText::new("ENGINE PHASES & LIFECYCLE").color(COLOR_PRIMARY));
    ui.label("Monitor engine readiness, task completion, and phase transitions.");
    ui.add_space(10.0);
    ui.separator();
    ui.add_space(10.0);

    let mut manager = world.resource_mut::<PhaseManager>();

    // 1. Timeline Visualization
    ui.horizontal(|ui| {
        for phase in &manager.phase_order {
            let is_current = *phase == manager.current_phase;
            let color = if is_current {
                Color32::GREEN
            } else {
                Color32::GRAY
            };
            let stroke = if is_current {
                egui::Stroke::new(2.0, Color32::WHITE)
            } else {
                egui::Stroke::NONE
            };

            ui.vertical(|ui| {
                ui.label(RichText::new(format!("{:?}", phase)).color(color));
                let (response, painter) =
                    ui.allocate_painter(egui::vec2(20.0, 20.0), egui::Sense::hover());
                let center = response.rect.center();
                painter.circle_filled(center, 8.0, color);
                if is_current {
                    painter.circle_stroke(center, 10.0, stroke);
                }
            });

            if phase != manager.phase_order.last().unwrap() {
                ui.label("➜");
            }
        }
    });

    ui.add_space(20.0);
    ui.separator();

    // 2. Current Status & Tasks
    ui.columns(2, |cols| {
        // Column A: Status
        cols[0].group(|ui| {
            ui.label(RichText::new("STATUS").strong());
            ui.separator();
            ui.horizontal(|ui| {
                ui.label("Current Phase:");
                ui.label(
                    RichText::new(format!("{:?}", manager.current_phase))
                        .strong()
                        .color(Color32::WHITE),
                );
            });

            ui.add_space(10.0);
            ui.label("Pending Tasks:");
            if manager.pending_tasks.is_empty() {
                ui.label(
                    RichText::new("All tasks complete.")
                        .italics()
                        .color(Color32::GREEN),
                );
            } else {
                for (task, status) in &manager.pending_tasks {
                    ui.horizontal(|ui| {
                        let (icon, color) = match status {
                            TaskStatus::Pending => ("⏳", Color32::YELLOW),
                            TaskStatus::InProgress => ("🔄", Color32::LIGHT_BLUE),
                            TaskStatus::Completed => ("✅", Color32::GREEN),
                            TaskStatus::Failed => ("❌", Color32::RED),
                        };
                        ui.label(icon);
                        ui.label(RichText::new(task).color(color));

                        if *status == TaskStatus::Failed && task == "Core Assets" {
                            ui.label(
                                RichText::new("(Run './dj gen')")
                                    .small()
                                    .color(Color32::GRAY),
                            );
                        }
                    });
                }
            }

            // Manual Controls (Debug)
            ui.add_space(20.0);
            ui.label("Debug Transitions:");
            ui.horizontal(|ui| {
                if ui.button("Next Phase").clicked() {
                    // Logic to find next phase
                    let current_idx = manager
                        .phase_order
                        .iter()
                        .position(|p| *p == manager.current_phase)
                        .unwrap_or(0);
                    if current_idx + 1 < manager.phase_order.len() {
                        let next_phase = manager.phase_order[current_idx + 1];
                        manager.set_phase(next_phase);
                    }
                }
                if ui.button("Reset to Loading").clicked() {
                    manager.set_phase(GamePhase::Loading);
                }
            });
        });

        // Column B: Event Log
        cols[1].group(|ui| {
            ui.label(RichText::new("PHASE LOG").strong());
            ui.separator();
            egui::ScrollArea::vertical()
                .id_salt("phase_log")
                .max_height(300.0)
                .show(ui, |ui| {
                    for msg in &manager.event_log {
                        ui.label(RichText::new(msg).monospace().size(10.0));
                    }
                });
        });
    });
}
