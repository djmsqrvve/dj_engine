//! DJ Engine Core - Master plugin bundle
//!
//! Provides `DJEnginePlugin` which bundles all engine systems for easy integration.

use bevy::prelude::*;

pub mod phases;

pub use phases::{GamePhase, GamePhasePlugin, PhaseManager};

use crate::types::EngineConfig;

/// Master plugin that bundles all DJ Engine systems.
///
/// # Example
/// ```ignore
/// use dj_engine::prelude::*;
///
/// App::new()
///     .add_plugins(DefaultPlugins)
///     .add_plugins(DJEnginePlugin::default())
///     .run();
/// ```
pub struct DJEnginePlugin {
    /// Engine configuration
    pub config: EngineConfig,
    /// Whether to include diagnostics overlay
    pub with_diagnostics: bool,
}

impl Default for DJEnginePlugin {
    fn default() -> Self {
        Self {
            config: EngineConfig::default(),
            with_diagnostics: true,
        }
    }
}

impl Plugin for DJEnginePlugin {
    fn build(&self, app: &mut App) {
        // Ensure state management is available first (critical for tests/minimal plugins)
        if !app.is_plugin_added::<bevy::state::app::StatesPlugin>() {
            app.add_plugins(bevy::state::app::StatesPlugin);
        }

        // Insert engine configuration
        app.insert_resource(self.config.clone());

        // Add core engine plugins
        app.add_plugins(phases::GamePhasePlugin)
           .add_plugins(crate::assets::DJAssetPlugin)
           .add_plugins(crate::input::DJInputPlugin)
           .add_plugins(crate::audio::DJAudioPlugin)
           .add_plugins(crate::animation::DJAnimationPlugin)
           .add_plugins(crate::scene::DJScenePlugin)
           .add_plugins(crate::story_graph::StoryGraphPlugin)
           .add_plugins(crate::physics::DJPhysicsPlugin)
           .add_plugins(crate::midi::MidiPlugin)
           .add_plugins(crate::lua_scripting::DJScriptingPlugin);

        if self.with_diagnostics {
            app.add_plugins(crate::diagnostics::DiagnosticsPlugin);
        }

        info!(
            "DJ Engine v{} initialized ({}x{}, debug={})",
            crate::engine_version(),
            self.config.internal_width,
            self.config.internal_height,
            self.config.debug_mode
        );
    }
}

impl DJEnginePlugin {
    /// Create a new engine plugin with custom configuration.
    pub fn new(config: EngineConfig) -> Self {
        Self {
            config,
            with_diagnostics: true,
        }
    }

    /// Disable the diagnostics overlay.
    pub fn without_diagnostics(mut self) -> Self {
        self.with_diagnostics = false;
        self
    }
}
