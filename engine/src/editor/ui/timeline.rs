//! DAW-style Timeline View for Game Logic and Event Buses

use bevy::prelude::*;
use bevy_egui::egui::{self, Color32, Pos2, Rect, Stroke, Vec2};

pub fn draw_timeline_view(ui: &mut egui::Ui, _world: &mut World) {
    let rect = ui.available_rect_before_wrap();
    let painter = ui.painter();

    // 1. Background
    painter.rect_filled(rect, 0.0, Color32::from_rgb(15, 15, 20));

    // VISUAL DEMO banner
    let banner_rect = Rect::from_min_size(rect.left_top(), Vec2::new(rect.width(), 24.0));
    painter.rect_filled(banner_rect, 0.0, Color32::from_rgb(20, 60, 80));
    painter.text(
        banner_rect.center(),
        egui::Align2::CENTER_CENTER,
        "🎹 TIMELINE DEMO — Logic Flow as Audio Tracks",
        egui::FontId::proportional(12.0),
        Color32::from_rgb(100, 200, 255),
    );

    // Layout Constants
    let header_width = 200.0;
    let ruler_height = 30.0;
    let track_height = 60.0;
    let start_y = rect.top() + 30.0 + ruler_height; // Below banner and ruler

    // 2. Time Ruler
    let ruler_rect = Rect::from_min_size(
        rect.left_top() + Vec2::new(header_width, 24.0),
        Vec2::new(rect.width() - header_width, ruler_height),
    );
    painter.rect_filled(ruler_rect, 0.0, Color32::from_rgb(25, 25, 35));

    for i in 0..20 {
        let x = ruler_rect.left() + (i as f32 * 100.0);
        if x < ruler_rect.right() {
            painter.line_segment(
                [
                    Pos2::new(x, ruler_rect.bottom()),
                    Pos2::new(x, ruler_rect.bottom() - 10.0),
                ],
                Stroke::new(1.0, Color32::GRAY),
            );
            painter.text(
                Pos2::new(x + 5.0, ruler_rect.center().y),
                egui::Align2::LEFT_CENTER,
                format!("00:0{}.00", i),
                egui::FontId::monospace(10.0),
                Color32::GRAY,
            );
        }
    }

    // 3. Tracks (Hardcoded Demo Data)
    struct Track {
        name: String,
        color: Color32,
        clips: Vec<(f32, f32, String)>, // start, width, name
    }

    let tracks = [Track {
            name: "🎮 INPUT BUS".into(),
            color: Color32::from_rgb(0, 150, 255),
            clips: vec![
                (50.0, 100.0, "Move Input".into()),
                (300.0, 50.0, "Jump".into()),
            ],
        },
        Track {
            name: "⚙️ PHYSICS".into(),
            color: Color32::from_rgb(255, 100, 0),
            clips: vec![
                (55.0, 100.0, "Velocity".into()),
                (305.0, 150.0, "Gravity Calc".into()),
            ],
        },
        Track {
            name: "🧠 LOGIC (QUEST)".into(),
            color: Color32::from_rgb(200, 0, 255),
            clips: vec![(400.0, 200.0, "Check Valid".into())],
        },
        Track {
            name: "💥 VFX / AUDIO".into(),
            color: Color32::from_rgb(0, 255, 100),
            clips: vec![
                (310.0, 80.0, "Jump SFX".into()),
                (450.0, 100.0, "Sparkle".into()),
            ],
        }];

    let mut y = start_y;

    // Render Tracks
    for (idx, track) in tracks.iter().enumerate() {
        let track_rect = Rect::from_min_size(
            Pos2::new(rect.left(), y),
            Vec2::new(rect.width(), track_height),
        );

        // Track Background styling
        if idx % 2 == 0 {
            painter.rect_filled(track_rect, 0.0, Color32::from_white_alpha(5));
        }

        // Track Header
        let header_rect =
            Rect::from_min_size(track_rect.left_top(), Vec2::new(header_width, track_height));
        painter.rect_filled(header_rect, 0.0, Color32::from_rgb(30, 30, 40));
        painter.line_segment(
            [header_rect.right_top(), header_rect.right_bottom()],
            Stroke::new(1.0, Color32::from_rgb(60, 60, 70)),
        );

        // Track Controls
        let name_pos = header_rect.left_top() + Vec2::new(10.0, 10.0);
        painter.text(
            name_pos,
            egui::Align2::LEFT_TOP,
            &track.name,
            egui::FontId::proportional(14.0),
            track.color,
        );

        // Mute/Solo buttons (visual only)
        let btn_y = name_pos.y + 25.0;
        painter.rect_stroke(
            Rect::from_min_size(Pos2::new(name_pos.x, btn_y), Vec2::new(20.0, 20.0)),
            2.0,
            Stroke::new(1.0, Color32::GRAY),
            egui::StrokeKind::Inside,
        );
        painter.text(
            Pos2::new(name_pos.x + 10.0, btn_y + 10.0),
            egui::Align2::CENTER_CENTER,
            "M",
            egui::FontId::monospace(10.0),
            Color32::GRAY,
        );

        painter.rect_stroke(
            Rect::from_min_size(Pos2::new(name_pos.x + 25.0, btn_y), Vec2::new(20.0, 20.0)),
            2.0,
            Stroke::new(1.0, Color32::GRAY),
            egui::StrokeKind::Inside,
        );
        painter.text(
            Pos2::new(name_pos.x + 35.0, btn_y + 10.0),
            egui::Align2::CENTER_CENTER,
            "S",
            egui::FontId::monospace(10.0),
            Color32::GRAY,
        );

        // Clips
        let timeline_start = header_rect.right();

        for (start_offset, width, name) in &track.clips {
            let clip_rect = Rect::from_min_size(
                Pos2::new(timeline_start + start_offset, y + 5.0),
                Vec2::new(*width, track_height - 10.0),
            );

            painter.rect_filled(clip_rect, 4.0, track.color.linear_multiply(0.3));
            painter.rect_stroke(clip_rect, 4.0, Stroke::new(1.0, track.color), egui::StrokeKind::Inside);

            painter.text(
                clip_rect.left_center() + Vec2::new(5.0, 0.0),
                egui::Align2::LEFT_CENTER,
                name,
                egui::FontId::proportional(12.0),
                Color32::WHITE,
            );
        }

        y += track_height + 2.0;
    }

    // 4. Signal Routing (Wires)
    // Connecting Input -> Physics -> Audio
    let wire_stroke = Stroke::new(3.0, Color32::from_rgb(255, 255, 100).linear_multiply(0.5));

    // Wire 1: Input Jump -> Physics Gravity
    let p1 = Pos2::new(header_width + 350.0, start_y + track_height - 10.0); // Input Jump Bottom
    let p2 = Pos2::new(header_width + 350.0, start_y + track_height + 10.0); // Physics Gravity Top
    painter.line_segment([p1, p2], wire_stroke);
    painter.circle_filled(p1, 4.0, Color32::YELLOW);
    painter.circle_filled(p2, 4.0, Color32::YELLOW);

    // Wire 2: Physics -> Audio
    let p3 = Pos2::new(header_width + 380.0, start_y + track_height * 2.0 - 10.0); // Physics Bottom
    let p4 = Pos2::new(header_width + 380.0, start_y + track_height * 3.0 + 10.0); // Audio Top (skipping logic)

    // Bezier curve for skipping a track
    let cp1 = p3 + Vec2::new(0.0, 50.0);
    let cp2 = p4 - Vec2::new(0.0, 50.0);

    use bevy_egui::egui::Shape;
    let curve = egui::epaint::CubicBezierShape::from_points_stroke(
        [p3, cp1, cp2, p4],
        false,
        Color32::TRANSPARENT,
        wire_stroke,
    );
    painter.add(Shape::CubicBezier(curve));
    painter.circle_filled(p3, 4.0, Color32::YELLOW);
    painter.circle_filled(p4, 4.0, Color32::YELLOW);
}
