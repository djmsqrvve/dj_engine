//! Engine diagnostics and performance monitoring.

use std::collections::HashMap;

use crate::types::DiagnosticConfig;
use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

pub mod console;
pub mod inspector;

/// Plugin that provides diagnostic overlays and performance tracking.
pub struct DiagnosticsPlugin;

#[derive(Resource, Default, Debug, Clone, Reflect)]
#[reflect(Resource)]
pub struct SystemTimer {
    pub timings: HashMap<String, f32>,
}

impl Plugin for DiagnosticsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DiagnosticConfig>()
            .init_resource::<SystemTimer>()
            .register_type::<DiagnosticConfig>()
            .register_type::<SystemTimer>()
            .add_plugins(FrameTimeDiagnosticsPlugin::default())
            .add_plugins(inspector::InspectorPlugin)
            .add_plugins(console::ConsolePlugin)
            .add_systems(Startup, setup_diagnostic_overlay)
            .add_systems(Update, (
                update_diagnostic_overlay,
                update_config_from_input,
                console_fps_logger_system.run_if(resource_exists::<DiagnosticConfig>),
            ));
    }
}

/// Marker for the diagnostic text UI.
#[derive(Component)]
struct DiagnosticText;

/// Marker for the diagnostic overlay root.
#[derive(Component)]
struct DiagnosticOverlay;

fn setup_diagnostic_overlay(mut commands: Commands, config: Res<DiagnosticConfig>) {
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(5.0),
                left: Val::Px(5.0),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            Visibility::Inherited,
            DiagnosticOverlay,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("Diagnostics Initializing..."),
                TextFont {
                    font_size: config.font_size,
                    ..default()
                },
                TextColor(config.text_color),
                DiagnosticText,
            ));
        });
}

fn update_config_from_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut config: ResMut<DiagnosticConfig>,
    mut query: Query<&mut Visibility, With<DiagnosticOverlay>>,
) {
    if keys.just_pressed(KeyCode::F3) {
        config.enabled = !config.enabled;
        for mut vis in &mut query {
            *vis = if config.enabled {
                Visibility::Inherited
            } else {
                Visibility::Hidden
            };
        }
        info!("Diagnostics toggled: {}", config.enabled);
    }
}

fn update_diagnostic_overlay(
    _time: Res<Time>,
    mut text_query: Query<&mut Text, With<DiagnosticText>>,
    config: Res<DiagnosticConfig>,
    diagnostics: Res<DiagnosticsStore>,
    timer: Res<SystemTimer>,
) {
    if !config.enabled {
        for mut text in &mut text_query {
            text.0 = String::new();
        }
        return;
    }

    let fps = diagnostics
        .get(&FrameTimeDiagnosticsPlugin::FPS)
        .and_then(|diag| diag.smoothed())
        .map(|value| format!("{:.1}", value))
        .unwrap_or_else(|| "N/A".into());

    let mut timing_str = String::new();
    let mut keys: Vec<_> = timer.timings.keys().collect();
    keys.sort();
    for key in keys {
        timing_str.push_str(&format!("\n{}: {:.2}ms", key, timer.timings[key] * 1000.0));
    }

    for mut text in &mut text_query {
        text.0 = format!("FPS: {}{}", fps, timing_str);
    }
}

/// Periodic console output for remote performance monitoring.
fn console_fps_logger_system(
    time: Res<Time>,
    diagnostics: Res<DiagnosticsStore>,
    mut last_log: Local<f32>,
    window: Single<&Window>,
) {
    if time.elapsed_secs() - *last_log > 5.0 {
        let fps = diagnostics
            .get(&FrameTimeDiagnosticsPlugin::FPS)
            .and_then(|diag| diag.smoothed())
            .unwrap_or(0.0);

        let (w, h) = (window.width() as i32, window.height() as i32);

        info!("Performance: {:.1} FPS | Window: {}x{}", fps, w, h);
        *last_log = time.elapsed_secs();
    }
}
