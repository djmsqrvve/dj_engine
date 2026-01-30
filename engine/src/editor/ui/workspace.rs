use bevy::prelude::*;
use bevy_egui::egui::{self, Color32, RichText};
use super::super::state::*;

pub fn draw_workspace_canvas(ui: &mut egui::Ui, world: &mut World) {
    let (mut workspace, active_idx) = {
        let mut state = world.resource_mut::<EditorUiState>();
        let active_idx = state.active_branch_idx;
        let ws = if let Some(branch) = state.active_branches.get_mut(active_idx) {
            branch.workspace.clone()
        } else {
            state.workspace.clone()
        };
        (ws, active_idx)
    };

    // 0. AUTO-INITIALIZE DEFAULT LAYOUT
    if workspace.elements.is_empty() {
        workspace.elements.push(StudioElement {
            id: "core_dashboard".into(),
            view: EditorView::Core,
            rect: egui::Rect::from_min_size(egui::pos2(-400.0, -300.0), egui::vec2(800.0, 600.0)),
            z_index: 0,
            is_minimized: false,
        });
        workspace.elements.push(StudioElement {
            id: "narrative_graph".into(),
            view: EditorView::StoryGraph,
            rect: egui::Rect::from_min_size(egui::pos2(450.0, -300.0), egui::vec2(600.0, 400.0)),
            z_index: 1,
            is_minimized: false,
        });
    }

    let rect = ui.available_rect_before_wrap();
    
    // 1. INPUT: Panning & Zooming
    if ui.input(|i| i.pointer.button_down(egui::PointerButton::Middle)) || 
       (ui.input(|i| i.modifiers.alt) && ui.input(|i| i.pointer.primary_down())) {
        let delta = ui.input(|i| i.pointer.delta());
        workspace.camera_pan += Vec2::new(delta.x, delta.y);
    }
    
    let (zoom_delta, _zoom_center) = ui.input(|i| (i.smooth_scroll_delta.y, i.pointer.hover_pos()));
    if zoom_delta != 0.0 {
        workspace.camera_zoom = (workspace.camera_zoom * (1.0 + zoom_delta * 0.002)).clamp(0.05, 5.0);
    }

    // 2. BACKGROUND GRID
    let painter = ui.painter();
    painter.rect_filled(rect, 0.0, Color32::from_rgb(10, 10, 15));
    
    let grid_size = 100.0 * workspace.camera_zoom;
    let grid_color = Color32::from_rgba_unmultiplied(60, 60, 80, 25);
    
    let offset_x = workspace.camera_pan.x % grid_size;
    let offset_y = workspace.camera_pan.y % grid_size;
    
    let mut x = rect.left() + offset_x;
    while x < rect.right() + grid_size {
        painter.line_segment([egui::pos2(x, rect.top()), egui::pos2(x, rect.bottom())], (1.0, grid_color));
        x += grid_size;
    }
    
    let mut y = rect.top() + offset_y;
    while y < rect.bottom() + grid_size {
        painter.line_segment([egui::pos2(rect.left(), y), egui::pos2(rect.right(), y)], (1.0, grid_color));
        y += grid_size;
    }

    // 3. DRAW STUDIO ELEMENTS
    let world_center = rect.center() + egui::vec2(workspace.camera_pan.x, workspace.camera_pan.y);
    let mut updated_elements = vec![];
    let mut elements_to_close = vec![];

    workspace.elements.sort_by_key(|e| e.z_index);
    
    for mut element in workspace.elements.drain(..) {
        let screen_pos = world_center + egui::vec2(element.rect.min.x, element.rect.min.y) * workspace.camera_zoom;
        let screen_size = element.rect.size() * workspace.camera_zoom;
        let scaled_rect = egui::Rect::from_min_size(screen_pos, screen_size);

        let mut is_open = true;
        let win_id = format!("{}##{}", get_view_name(&element.view), element.id);
        
        let win_resp = egui::Window::new(RichText::new(get_view_name(&element.view)).strong().color(COLOR_PRIMARY))
            .id(egui::Id::new(&win_id))
            .default_rect(scaled_rect)
            .collapsible(true)
            .resizable(true)
            .open(&mut is_open)
            .show(ui.ctx(), |ui| {
                draw_element_content(ui, world, element.view.clone());
            });
            
        if let Some(inner) = win_resp {
            let new_screen_rect = inner.response.rect;
            let min_vec = (new_screen_rect.min.to_vec2() - world_center.to_vec2()) / workspace.camera_zoom;
            element.rect = egui::Rect::from_min_size(
                egui::pos2(min_vec.x, min_vec.y),
                new_screen_rect.size() / workspace.camera_zoom
            );
            
            if inner.response.clicked() || inner.response.dragged() {
                workspace.active_element_id = Some(element.id.clone());
            }
        }

        if is_open {
            updated_elements.push(element);
        } else {
            elements_to_close.push(element.id);
        }
    }
    
    // 3.5 BRING ACTIVE TO FRONT
    if let Some(id) = &workspace.active_element_id {
        if let Some(idx) = updated_elements.iter().position(|e| &e.id == id) {
            let active = updated_elements.remove(idx);
            updated_elements.push(active);
            // Re-normalize Z-indices
            for (i, el) in updated_elements.iter_mut().enumerate() {
                el.z_index = i as u32;
            }
        }
    }
    
    workspace.elements = updated_elements;

    // 4. SYNC BACK
    {
        let mut state = world.resource_mut::<EditorUiState>();
        if let Some(branch) = state.active_branches.get_mut(active_idx) {
            branch.workspace = workspace;
        } else {
            state.workspace = workspace;
        }
    }
}

fn get_view_name(view: &EditorView) -> &str {
    match view {
        EditorView::Core => "📦 Core",
        EditorView::FeatureGrid => "🔷 Feature Grid",
        EditorView::Timeline => "🎹 Timeline",
        EditorView::MapEditor => "🗺 Map",
        EditorView::ScenarioEditor => "🎭 Scenario",
        EditorView::StoryGraph => "📽 Story Graph",
        EditorView::Campaign => "📅 Campaign",
        EditorView::Hierarchy => "📁 Hierarchy",
        EditorView::Palette => "🎨 Palette",
        EditorView::Inspector => "🔍 Inspector",
        EditorView::Console => "💻 Console",
        EditorView::Settings => "⚙ Settings",
        _ => "Tool",
    }
}

fn draw_element_content(ui: &mut egui::Ui, world: &mut World, view: EditorView) {
    match view {
        EditorView::Core => super::views::draw_core_dashboard(ui, world),
        EditorView::StoryGraph => super::views::draw_story_graph(ui, world),
        EditorView::Timeline => super::timeline::draw_timeline_view(ui, world),
        EditorView::FeatureGrid => super::feature_grid::draw_feature_grid(ui, world),
        EditorView::Hierarchy => {
           world.resource_scope::<EditorUiState, _>(|world, mut ui_state| {
                bevy_inspector_egui::bevy_inspector::hierarchy::hierarchy_ui(world, ui, &mut ui_state.selected_entities);
           });
        }
        EditorView::Palette => super::panels::draw_palette_content(ui),
        EditorView::Inspector => super::panels::draw_right_panel(ui, world),
        EditorView::Console => super::panels::draw_console_panel(ui, world),
        EditorView::Settings => super::views::draw_settings_view(ui, world),
        EditorView::Controls => super::views::draw_controls_view(ui, world),
        _ => { ui.label(format!("View {:?} Content Placeholder", view)); }
    }
}
