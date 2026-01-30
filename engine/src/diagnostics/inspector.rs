use crate::types::DiagnosticConfig;
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

pub struct InspectorPlugin;

impl Plugin for InspectorPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(
            WorldInspectorPlugin::new()
                .run_if(|config: Res<DiagnosticConfig>| config.show_inspector),
        );

        info!("Inspector Plugin initialized");
    }
}
