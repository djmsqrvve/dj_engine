use crate::data::loader;
use crate::data::components::Vec3Data;
use crate::data::story::{
    StoryNodeData, StoryNodeVariant, DialogueNodeData, ChoiceNodeData,
    ActionNodeData
};
use crate::editor::state::{EditorUiState, COLOR_PRIMARY, COLOR_SECONDARY, ActiveStoryGraph};
use bevy::prelude::*;
pub use bevy_egui::egui;
use bevy_egui::egui::{Color32, Stroke, RichText};

use super::super::state::*;

pub fn draw_core_dashboard(ui: &mut egui::Ui, world: &mut World) {
    let rect = ui.available_rect_before_wrap();

    // 1. Background
    ui.painter()
        .rect_filled(rect, 0.0, Color32::from_rgb(5, 5, 10));

    // 2. Draw the Celestial Placeholder (Under Construction)
    draw_celestial_dashboard_placeholder(ui, rect);

    // 3. Current Branch Visualization (Legend/Mini-map style)
    let ui_state = world.resource::<EditorUiState>();
    let branches = &ui_state.active_branches;
    let active_idx = ui_state.active_branch_idx;

    let mut switch_branch = None;

    // Legend Area (Bottom Left)
    let legend_rect = egui::Rect::from_min_size(
        rect.left_bottom() + egui::vec2(20.0, -150.0),
        egui::vec2(250.0, 130.0),
    );
    ui.painter()
        .rect_filled(legend_rect, 5.0, Color32::from_black_alpha(200));
    ui.painter()
        .rect_stroke(legend_rect, 5.0, Stroke::new(1.0, COLOR_PRIMARY.linear_multiply(0.2)), egui::StrokeKind::Inside);

    ui.scope_builder(egui::UiBuilder::new().max_rect(legend_rect.shrink(10.0)), |ui| {
        ui.label(RichText::new("ACTIVE TEST PLANES").strong().small());
        ui.separator();
        egui::ScrollArea::vertical().show(ui, |ui| {
            for (idx, branch) in branches.iter().enumerate() {
                let is_active = idx == active_idx;
                let text = if is_active {
                    format!("● {}", branch.name)
                } else {
                    branch.name.clone()
                };

                if ui.selectable_label(is_active, text).clicked() {
                    switch_branch = Some(idx);
                }
            }
        });
    });

    if let Some(idx) = switch_branch {
        if let Some(mut state) = world.get_resource_mut::<EditorUiState>() {
            state.active_branch_idx = idx;
        }
    }
}

fn draw_celestial_dashboard_placeholder(ui: &mut egui::Ui, rect: egui::Rect) {
    let painter = ui.painter();
    let center = rect.center();

    // -- RADIATING CONSTELLATION --
    let node_count = 12;
    for i in 0..node_count {
        let angle = (i as f32 / node_count as f32) * std::f32::consts::TAU;
        let dist = 180.0 + (i % 3) as f32 * 20.0;
        let pos = center + egui::vec2(angle.cos(), angle.sin()) * dist;

        // Connections to center
        painter.line_segment(
            [center, pos],
            (1.0, COLOR_PRIMARY.linear_multiply(0.15)),
        );

        // Sub-connections (flower pattern)
        let next_angle = ((i + 1) as f32 / node_count as f32) * std::f32::consts::TAU;
        let next_pos = center + egui::vec2(next_angle.cos(), next_angle.sin()) * (180.0 + ((i + 1) % 3) as f32 * 20.0);
        painter.line_segment(
            [pos, next_pos],
            (1.0, COLOR_SECONDARY.linear_multiply(0.1)),
        );

        // Nodes
        let color = if i % 2 == 0 { COLOR_PRIMARY } else { COLOR_SECONDARY };
        painter.circle_filled(pos, 3.0, color.linear_multiply(0.5));
        painter.circle_stroke(pos, 6.0, (1.0, color.linear_multiply(0.2)));
    }

    // Central Hub
    painter.circle_filled(center, 12.0, Color32::WHITE);
    painter.circle_stroke(center, 25.0, (2.0, COLOR_PRIMARY));

    // -- FLOATING CONSTRUCTION WINDOWS (Mimicking the image) --
    let box_style = |painter: &egui::Painter, r: egui::Rect, title: &str, color: Color32| {
        painter.rect_filled(r, 4.0, Color32::from_black_alpha(220));
        painter.rect_stroke(r, 4.0, Stroke::new(1.5, color.linear_multiply(0.4)), egui::StrokeKind::Inside);
        painter.rect_filled(r.with_max_y(r.min.y + 25.0), 4.0, color.linear_multiply(0.1));
        
        painter.text(r.min + egui::vec2(8.0, 5.0), egui::Align2::LEFT_TOP, title, egui::FontId::proportional(12.0), Color32::WHITE);
        
        // Fake Code Lines
        for j in 0..4 {
            let line_y = r.min.y + 35.0 + (j as f32 * 12.0);
            let width = 60.0 + (j % 3) as f32 * 40.0;
            painter.line_segment(
                [egui::pos2(r.min.x + 10.0, line_y), egui::pos2(r.min.x + 10.0 + width, line_y)],
                (2.0, Color32::from_white_alpha(30))
            );
        }
    };

    // Testing Branch A (Top Left)
    box_style(painter, egui::Rect::from_center_size(center + egui::vec2(-350.0, -180.0), egui::vec2(180.0, 100.0)), "Testing Branch A", COLOR_PRIMARY);
    // Testing Branch B (Bottom Left)
    box_style(painter, egui::Rect::from_center_size(center + egui::vec2(-380.0, 120.0), egui::vec2(180.0, 100.0)), "Testing Branch B", COLOR_PRIMARY);
    // Code Maintenance (Top Right)
    box_style(painter, egui::Rect::from_center_size(center + egui::vec2(350.0, -150.0), egui::vec2(180.0, 100.0)), "Code Maintenance", COLOR_SECONDARY);
    // Build Core (Bottom Right)
    let build_core_rect = egui::Rect::from_center_size(center + egui::vec2(380.0, 180.0), egui::vec2(200.0, 80.0));
    painter.rect_filled(build_core_rect, 4.0, Color32::from_black_alpha(220));
    painter.rect_stroke(build_core_rect, 4.0, Stroke::new(2.0, COLOR_PRIMARY), egui::StrokeKind::Inside);
    painter.text(build_core_rect.center(), egui::Align2::CENTER_CENTER, "BUILD CORE: PASSED", egui::FontId::proportional(14.0), COLOR_PRIMARY);

    // -- HUD LABELS --
    painter.text(
        rect.center_top() + egui::vec2(0.0, 40.0),
        egui::Align2::CENTER_TOP,
        "PARALLEL FEATURE ENGINE PROTOTYPE",
        egui::FontId::proportional(28.0),
        Color32::WHITE.linear_multiply(0.8),
    );

    painter.text(
        rect.center_bottom() + egui::vec2(0.0, -40.0),
        egui::Align2::CENTER_BOTTOM,
        "CELESTIAL VERSIONING SYSTEM [DEVELOPMENT ACTIVE]",
        egui::FontId::monospace(14.0),
        COLOR_SECONDARY,
    );
}

pub fn draw_grid(ui: &mut egui::Ui, world: &mut World) {
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

            let selected_item = world
                .resource::<EditorUiState>()
                .selected_palette_item
                .clone();

            if let Some(item) = selected_item {
                debug!(
                    "Editor: USER CLICK SPAWN: {} at ({}, {})",
                    item, snap_x, snap_y
                );

                // Determine color based on item
                let color = match item.as_str() {
                    "Grass" => Color::srgb(0.2, 0.8, 0.2),
                    "Wall" => Color::srgb(0.5, 0.5, 0.5),
                    "Hamster" => Color::srgb(0.8, 0.5, 0.2),
                    "Chest" => Color::srgb(0.8, 0.8, 0.1),
                    _ => Color::WHITE,
                };

                world.spawn((
                    LogicalEntity,
                    Name::new(format!("{} [{:.0}, {:.0}]", item, snap_x, snap_y)),
                    Sprite {
                        color,
                        custom_size: Some(Vec2::new(30.0, 30.0)),
                        ..default()
                    },
                    Transform::from_xyz(snap_x, snap_y, 0.0),
                ));
            }
        }
    }

    // 2. Draw Grid Visuals
    painter.rect_filled(rect, 0.0, COLOR_BG);

    if !world.resource::<EngineSettings>().draw_grid {
        return;
    }

    let grid_size = 40.0;
    let color = Color32::from_rgb(30, 30, 40);

    let mut x = rect.left();
    while x < rect.right() {
        painter.line_segment(
            [egui::pos2(x, rect.top()), egui::pos2(x, rect.bottom())],
            (1.0, color),
        );
        x += grid_size;
    }

    let mut y = rect.top();
    while y < rect.bottom() {
        painter.line_segment(
            [egui::pos2(rect.left(), y), egui::pos2(rect.right(), y)],
            (1.0, color),
        );
        y += grid_size;
    }

    // Draw ghost of selected item at mouse cursor
    if let Some(_item) = &world.resource::<EditorUiState>().selected_palette_item {
        if let Some(pointer_pos) = ui.input(|i| i.pointer.hover_pos()) {
            if rect.contains(pointer_pos) {
                painter.circle_filled(pointer_pos, 5.0, COLOR_PRIMARY);
            }
        }
    }
}

pub fn draw_story_graph(ui: &mut egui::Ui, world: &mut World) {
    let painter = ui.painter().clone();
    
    // Get runtime state for highlighting
    let executor = world.resource::<crate::story_graph::types::GraphExecutor>();
    let active_node_name = {
        let library = world.get_resource::<crate::story_graph::types::StoryGraphLibrary>();
        if let (Some(node_id), Some(graph_id), Some(lib)) = (executor.current_node, &executor.active_graph_id, library) {
            lib.graphs.get(graph_id).and_then(|g| g.node_names.get(&node_id)).cloned()
        } else {
            None
        }
    };

    // Toolbar & Breadcrumbs
    ui.horizontal(|ui| {
        // Simulation Link
        if ui.button("🚀 Simulator").on_hover_text("Launch Headless CLI Simulation").clicked() {
             info!("Launching Headless Simulation...");
        }

        ui.separator();

        // Breadcrumbs
        let mut ui_state = world.resource_mut::<EditorUiState>();
        if ui_state.graph_stack.is_empty() {
             ui.label(RichText::new("Main Graph").strong().color(COLOR_PRIMARY));
        } else {
            if ui.link("Main").clicked() {
                ui_state.graph_stack.clear();
            }
            for graph_name in ui_state.graph_stack.iter() {
                ui.label(">");
                if ui.link(graph_name).clicked() {
                    // Truncate stack
                    // (Actually we'd need to resize properly)
                }
            }
        }

        ui.add_space(20.0);
        
        if ui.button("📂 Load Test Game").clicked() {
            let path =
                std::path::PathBuf::from("games/dev/new_horizon/story_graphs/test_game.json");
            match loader::load_story_graph(&path) {
                Ok(loaded_graph) => {
                    world.resource_scope::<ActiveStoryGraph, _>(|_, mut graph| {
                        graph.0 = loaded_graph;
                        info!("Loaded test game successfully!");
                    });
                }
                Err(e) => error!("Failed to load test game: {}", e),
            }
        }
        ui.label(
            RichText::new("Use middle-click to pan, scroll to zoom")
                .italics()
                .color(Color32::GRAY),
        );
    });

    let rect = ui.available_rect_before_wrap();

    // 1. Draw Background & Grid
    painter.rect_filled(rect, 0.0, Color32::from_rgb(15, 15, 20));
    for i in 0..25 {
        let x = rect.min.x + (i as f32 * 80.0);
        painter.line_segment(
            [egui::pos2(x, rect.min.y), egui::pos2(x, rect.max.y)],
            (1.0, Color32::from_rgba_unmultiplied(60, 60, 80, 40)),
        );
        let y = rect.min.y + (i as f32 * 80.0);
        painter.line_segment(
            [egui::pos2(rect.min.x, y), egui::pos2(rect.max.x, y)],
            (1.0, Color32::from_rgba_unmultiplied(60, 60, 80, 40)),
        );
    }

    // Auto-init for empty graph
    world.resource_scope::<ActiveStoryGraph, _>(|_, mut graph| {
        if graph.0.nodes.is_empty() {
            let mut start = StoryNodeData::start("start", None::<String>);
            start.position = Vec3Data::new(100.0, 100.0, 0.0);
            graph.0.root_node_id = "start".to_string();
            graph.0.add_node(start);
        }
    });

    // Context Menu & Drop
    let mut add_node_cmd = None;

    // We can't access world inside context_menu closure easily if we are borrowing it from outside?
    // Egui context menu runs immediately.
    let response = ui.allocate_rect(rect, egui::Sense::click());

    // Handle Drop
    if ui.rect_contains_pointer(rect) && ui.input(|i| !i.pointer.primary_down()) {
        let mut ui_state = world.resource_mut::<EditorUiState>();
        if let Some(dragged_type) = ui_state.dragged_node_id.take() {
            add_node_cmd = Some(dragged_type); // Convert drag to add cmd
        }
    }

    response.context_menu(|ui| {
        ui.label(
            RichText::new("ADD STORY ELEMENT")
                .strong()
                .color(COLOR_PRIMARY),
        );
        ui.separator();
        if ui.button("🎬 Start").clicked() {
            add_node_cmd = Some("Start".to_string());
        }
        if ui.button("💬 Dialogue").clicked() {
            add_node_cmd = Some("Dialogue".to_string());
        }
        if ui.button("📦 Scene Container").clicked() {
            add_node_cmd = Some("SubGraph".to_string());
        }
        if ui.button("🔚 End").clicked() {
            add_node_cmd = Some("End".to_string());
        }
    });

    if let Some(cmd) = add_node_cmd {
        world.resource_scope::<ActiveStoryGraph, _>(|_, mut graph| {
            let id = format!("node_{}", graph.0.nodes.len());
            let pos = response.interact_pointer_pos().unwrap_or(rect.center());
            // Adjust to be relative to panel if needed, but we store absolute screen coords for simpler drag?
            // Ideally relative to rect.min.
            let rel_pos = pos - rect.min;

            let mut node = match cmd.as_str() {
                "Start" => StoryNodeData::start(id.clone(), None::<String>),
                "Dialogue" => StoryNodeData::dialogue(id.clone(), "Stranger", "Hello world"),
                "Choice" => {
                    let mut n = StoryNodeData::dialogue(id.clone(), "System", "Choice Node Placeholder");
                    n.data = StoryNodeVariant::Choice(ChoiceNodeData::default());
                    n
                }
                "Action" => {
                    let mut n = StoryNodeData::dialogue(id.clone(), "System", "Action Node Placeholder");
                    n.data = StoryNodeVariant::Action(ActionNodeData::default());
                    n
                }
                "Container" | "SubGraph" | "Scene Container" => {
                    let mut n = StoryNodeData::end(id.clone());
                    n.data = StoryNodeVariant::SubGraph(crate::data::story::SubGraphNodeData {
                        graph_id: "scene_01".to_string(),
                        next_node_id: None,
                    });
                    n
                }
                "Background" => StoryNodeData::end(id.clone()),
                "Camera" => StoryNodeData::end(id.clone()),
                "Wait" => StoryNodeData::end(id.clone()),
                "End" => StoryNodeData::end(id.clone()),
                _ => StoryNodeData::dialogue(id.clone(), "Err", format!("Unknown: {}", cmd)),
            };

            // Set position
            node.position = Vec3Data::new(rel_pos.x, rel_pos.y, 0.0);

            // If start, set root
            if cmd == "Start" {
                graph.0.root_node_id = id.clone();
            }

            graph.0.add_node(node);
        });
    }

    // DRAW NODES AND LINES
    // We need to scope world to get graph
    world.resource_scope::<ActiveStoryGraph, _>(|world, mut graph| {
        let library = world.get_resource::<crate::story_graph::types::StoryGraphLibrary>();
        let mut ui_state = world.resource_mut::<EditorUiState>();

        // 1. Draw Connections
        for node in &graph.0.nodes {
            let start_pos =
                rect.min + egui::vec2(node.position.x, node.position.y) + egui::vec2(100.0, 25.0); // Approx right side

            for next_id in node.next_node_ids() {
                if let Some(target) = graph.0.find_node(next_id) {
                    let end_pos = rect.min
                        + egui::vec2(target.position.x, target.position.y)
                        + egui::vec2(0.0, 25.0); // Approx left side
                    painter.line_segment([start_pos, end_pos], (2.0, Color32::GRAY));
                }
            }

            // Draw active drag line
            if let Some(start_id) = &ui_state.connection_start_id {
                if start_id == &node.id {
                    if let Some(pointer) = ui.input(|i| i.pointer.hover_pos()) {
                        painter.line_segment([start_pos, pointer], (2.0, Color32::YELLOW));
                    }
                }
            }
        }

        // 2. Draw Nodes
        let mut node_to_update_pos = None;
        let mut connection_established = None; // (from, to)

        for node in &mut graph.0.nodes {
            let node_rect = egui::Rect::from_min_size(
                rect.min + egui::vec2(node.position.x, node.position.y),
                egui::vec2(150.0, 80.0),
            );

            let (bg, stroke) = match &node.data {
                StoryNodeVariant::Start(_) => {
                    (Color32::from_rgb(0, 80, 40), Color32::GREEN)
                }
                StoryNodeVariant::End(_) => {
                    (Color32::from_rgb(80, 0, 0), Color32::RED)
                }
                StoryNodeVariant::Dialogue(_) => {
                    (Color32::from_rgb(0, 40, 100), COLOR_PRIMARY)
                }
                StoryNodeVariant::SubGraph(_) => {
                    (Color32::from_rgb(60, 20, 80), Color32::GOLD)
                }
                StoryNodeVariant::Action(_) => {
                    (Color32::from_rgb(80, 60, 0), Color32::YELLOW)
                }
                StoryNodeVariant::Choice(_) => {
                    (Color32::from_rgb(0, 80, 100), COLOR_SECONDARY)
                }
                StoryNodeVariant::Conditional(_) => {
                    (Color32::from_rgb(100, 40, 0), Color32::ORANGE)
                }
                StoryNodeVariant::SetFlag(_) => {
                    (Color32::from_rgb(40, 80, 0), Color32::LIGHT_GREEN)
                }
                _ => (Color32::from_rgb(40, 40, 40), Color32::GRAY),
            };

            painter.rect_filled(node_rect, 6.0, bg);
            
            // 1. Trace Highlight (Recent Path)
            if let Some(pos) = ui_state.node_trace.iter().position(|id| id == &node.id) {
                let age = (ui_state.node_trace.len() - 1 - pos) as f32;
                let alpha = (1.0 - (age / 5.0).min(0.8)) * 255.0;
                let trace_color = Color32::from_rgba_unmultiplied(255, 255, 0, alpha as u8);
                painter.rect_stroke(node_rect, 6.0, Stroke::new(2.0, trace_color), egui::StrokeKind::Inside);
            }

            // 2. Active Node Highlight (Current)
            if Some(&node.id) == active_node_name.as_ref() {
                 painter.rect_stroke(node_rect.expand(4.0), 10.0, Stroke::new(4.0, Color32::YELLOW), egui::StrokeKind::Inside);
            }

            // Double stroke for SubGraph (Container)
            if matches!(
                node.data,
                StoryNodeVariant::SubGraph(_)
            ) {
                painter.rect_stroke(
                    node_rect.shrink(3.0),
                    4.0,
                    Stroke::new(1.0, stroke.linear_multiply(0.5)),
                    egui::StrokeKind::Inside,
                );
            }
            painter.rect_stroke(node_rect, 6.0, Stroke::new(1.5, stroke), egui::StrokeKind::Inside);

            painter.text(
                node_rect.min + egui::vec2(10.0, 10.0),
                egui::Align2::LEFT_TOP,
                &node.id,
                egui::FontId::proportional(14.0),
                Color32::WHITE,
            );
            painter.text(
                node_rect.min + egui::vec2(10.0, 30.0),
                egui::Align2::LEFT_TOP,
                format!("{:?}", node.node_type()),
                egui::FontId::proportional(11.0),
                Color32::LIGHT_GRAY,
            );

            // Ports Labels
            let _is_start = matches!(node.node_type(), crate::data::story::StoryNodeType::Start);
            let is_end = matches!(node.node_type(), crate::data::story::StoryNodeType::End);

            // Input Label
            match &node.data {
                StoryNodeVariant::Dialogue(d) => {
                    painter.text(
                        node_rect.left_top() + egui::vec2(10.0, 10.0),
                        egui::Align2::LEFT_TOP,
                        &d.speaker_id,
                        egui::FontId::proportional(14.0),
                        Color32::WHITE,
                    );
                }
                StoryNodeVariant::Choice(c) => {
                    painter.text(
                        node_rect.left_top() + egui::vec2(10.0, 10.0),
                        egui::Align2::LEFT_TOP,
                        format!("? {}", c.options.len()),
                        egui::FontId::proportional(14.0),
                        Color32::WHITE,
                    );
                }
                _ => {}
            }
            // Input visual dot
            painter.circle_filled(node_rect.left_center(), 3.0, Color32::GRAY);

            // Output Label (Right) - Not for End
            if !is_end {
                painter.text(
                    node_rect.right_center() - egui::vec2(15.0, 0.0), // Shift left of the connect button
                    egui::Align2::RIGHT_CENTER,
                    "Out",
                    egui::FontId::monospace(10.0),
                    Color32::GRAY,
                );
            }

            // Interaction
            let response = ui.allocate_rect(node_rect, egui::Sense::drag());
            if response.dragged() {
                node_to_update_pos = Some((node.id.clone(), response.drag_delta()));
            }
            if response.clicked() {
                ui_state.selected_node_id = Some(node.id.clone());
            }

            // Connect Button (Little circle on right)
            let port_rect =
                egui::Rect::from_center_size(node_rect.right_center(), egui::vec2(12.0, 12.0));
            painter.circle_filled(port_rect.center(), 6.0, Color32::WHITE);
            let port_resp = ui.allocate_rect(port_rect, egui::Sense::click());

            if port_resp.clicked() {
                if let Some(start_id) = ui_state.connection_start_id.clone() {
                    // Complete connection
                    if start_id != node.id {
                        connection_established = Some((start_id, node.id.clone()));
                        ui_state.connection_start_id = None;
                    }
                } else {
                    // Start connection
                    ui_state.connection_start_id = Some(node.id.clone());
                }
            }

            // If clicking node body while connecting, also connect (easier target)
            if response.clicked() && ui_state.connection_start_id.is_some() {
                if let Some(start_id) = &ui_state.connection_start_id {
                    if start_id != &node.id {
                        connection_established = Some((start_id.clone(), node.id.clone()));
                        ui_state.connection_start_id = None;
                    }
                }
            }
        }

        // Apply position updates
        if let Some((id, delta)) = node_to_update_pos {
            if let Some(node) = graph.0.nodes.iter_mut().find(|n| n.id == id) {
                node.position.x += delta.x;
                node.position.y += delta.y;
            }
        }

        // Apply connection
        if let Some((from, to)) = connection_established {
            if let Some(node) = graph.0.nodes.iter_mut().find(|n| n.id == from) {
                // Ugly mutation manually based on type
                // TODO(#109): Add helper 'set_next' to StoryNodeData to avoid manual variant matching
                match &mut node.data {
                    StoryNodeVariant::Start(d) => d.next_node_id = Some(to),
                    StoryNodeVariant::Dialogue(d) => d.next_node_id = Some(to),
                    StoryNodeVariant::Action(a) => a.next_node_id = Some(to),
                    StoryNodeVariant::SubGraph(s) => s.next_node_id = Some(to),
                    _ => {}
                }
            }
        }
    });
}

pub fn draw_controls_view(ui: &mut egui::Ui, world: &mut World) {
    use crate::input::InputConfig;

    ui.heading(RichText::new("ENGINE CONTROLS").color(COLOR_PRIMARY));
    ui.add_space(10.0);
    ui.separator();
    ui.add_space(10.0);

    let config = world.resource::<InputConfig>();

    egui::Grid::new("controls_grid")
        .num_columns(2)
        .spacing([40.0, 10.0])
        .striped(true)
        .show(ui, |ui| {
            ui.label(RichText::new("ACTION").strong());
            ui.label(RichText::new("KEYS").strong());
            ui.end_row();

            // Group by Action (BTreeMap for deterministic order)
            let mut actions: std::collections::BTreeMap<crate::input::InputAction, Vec<String>> =
                std::collections::BTreeMap::new();
            for (key, action) in &config.keyboard {
                actions
                    .entry(*action)
                    .or_default()
                    .push(key.clone());
            }

            for (action, keys) in actions {
                ui.label(format!("{:?}", action));
                ui.label(keys.join(", "));
                ui.end_row();
            }
        });

    ui.add_space(20.0);
    ui.label(
        RichText::new("TIP: You can edit these in engine/src/input/mod.rs (for now).")
            .italics()
            .color(Color32::GRAY),
    );
}

pub fn draw_settings_view(ui: &mut egui::Ui, world: &mut World) {
    ui.heading(RichText::new("ENGINE SETTINGS").color(COLOR_PRIMARY));
    ui.add_space(10.0);
    ui.separator();
    ui.add_space(10.0);

    egui::ScrollArea::vertical().show(ui, |ui| {
        ui.columns(2, |cols| {
            // COLUMN 1: General & Audio
            cols[0].group(|ui| {
                ui.label(RichText::new("📁 GENERAL").strong().color(COLOR_PRIMARY));
                ui.separator();
                let project = world.resource::<ProjectMetadata>();
                ui.label(format!("Project: {}", project.name));
                ui.label(format!(
                    "Path: {:?}",
                    project
                        .path
                        .as_ref()
                        .unwrap_or(&std::path::PathBuf::from("None"))
                ));
                ui.add_space(10.0);
                if ui.button("Clean Cache").clicked() { /* TODO(#110): Implement cache cleanup */ }
            });

            cols[0].add_space(10.0);

            let mut settings = world.resource_mut::<EngineSettings>();
            let initial_settings = settings.clone();

            cols[0].group(|ui| {
                ui.label(RichText::new("🔊 AUDIO").strong().color(COLOR_PRIMARY));
                ui.separator();
                ui.label("Master Volume");
                ui.add(egui::Slider::new(&mut settings.master_volume, 0.0..=1.0));
                ui.checkbox(&mut true, "Mute on focus loss");
            });

            // COLUMN 2: Graphics & Dev
            cols[1].group(|ui| {
                ui.label(RichText::new("📺 GRAPHICS").strong().color(COLOR_PRIMARY));
                ui.separator();
                ui.label("Resolution Profile: Auto");
                ui.checkbox(&mut true, "VSync Enabled");
                ui.checkbox(&mut false, "CRT Post-process");
            });

            cols[1].add_space(10.0);

            cols[1].group(|ui| {
                ui.label(
                    RichText::new("🛠 DEV SETTINGS")
                        .strong()
                        .color(COLOR_SECONDARY),
                );
                ui.separator();
                ui.label(
                    RichText::new("Experimental features and debug tools")
                        .small()
                        .italics(),
                );

                ui.add_space(5.0);
                ui.checkbox(&mut settings.show_bounds, "Show Entity Bounds");
                ui.checkbox(&mut settings.draw_grid, "Draw Grid Overlay");
                ui.checkbox(&mut settings.log_scripts, "Log Script events");

                ui.add_space(10.0);
                if ui
                    .button(RichText::new("RESET ENGINE").color(Color32::RED))
                    .clicked()
                {
                    *settings = EngineSettings::default();
                }
            });

            cols[0].add_space(10.0);

            // DISPLAY SETTINGS
            cols[0].group(|ui| {
                ui.label(RichText::new("🖥️ DISPLAY").strong().color(COLOR_PRIMARY));
                ui.separator();

                ui.horizontal(|ui| {
                    ui.label("Resolution:");
                    if ui.button("1920x1080").clicked() {
                        settings.window_width = 1920.0;
                        settings.window_height = 1080.0;
                    }
                    if ui.button("1280x720").clicked() {
                        settings.window_width = 1280.0;
                        settings.window_height = 720.0;
                    }
                });

                ui.horizontal(|ui| {
                    ui.label("Mode:");
                    egui::ComboBox::from_id_salt("window_mode")
                        .selected_text(match settings.window_mode_index {
                            1 => "Borderless",
                            2 => "Fullscreen",
                            _ => "Windowed",
                        })
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut settings.window_mode_index, 0, "Windowed");
                            ui.selectable_value(&mut settings.window_mode_index, 1, "Borderless");
                            ui.selectable_value(&mut settings.window_mode_index, 2, "Fullscreen");
                        });
                });

                ui.horizontal(|ui| {
                    ui.label("Monitor Index:");
                    ui.add(egui::DragValue::new(&mut settings.monitor_index).range(0..=4));
                });
            });

            // Auto-save if settings changed
            if *settings != initial_settings {
                if let Err(e) = settings.save() {
                    error!("Failed to save settings: {}", e);
                }
            }
        });
    });
}
