//! Project-level configuration and settings.
//!
//! The [`Project`] struct is the top-level container for an entire game project,
//! referencing all scenes, story graphs, databases, and assets.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use bevy::prelude::*;

/// Input profile for the game (determines default control schemes).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InputProfile {
    /// JRPG-style keyboard/gamepad input
    #[default]
    Jrpg,
    /// RTS-style mouse/keyboard input
    Rts,
    /// Hybrid input supporting both styles
    Hybrid,
}

/// Editor theme preference.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EditorTheme {
    Light,
    #[default]
    Dark,
}

/// Editor gizmo mode for transform manipulation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GizmoMode {
    #[default]
    Move,
    Rotate,
    Scale,
}

/// Editor layout preset.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LayoutPreset {
    /// Optimized for JRPG map editing
    #[default]
    JrpgMapping,
    /// Optimized for TD balance/wave tuning
    TdBalancing,
    /// Custom user-defined layout
    Custom,
}

/// 2D size with integer dimensions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct Size2i {
    pub width: u32,
    pub height: u32,
}

/// 2D size with float dimensions.
#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
pub struct Size2f {
    pub x: f32,
    pub y: f32,
}

/// Localization settings.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct LocalizationSettings {
    /// Available languages (e.g., ["en", "fr", "jp"])
    pub languages: Vec<String>,
    /// Default language code
    pub default_language: String,
}

/// File path configuration for project assets.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectPaths {
    /// Path to scenes directory
    pub scenes: String,
    /// Path to story graphs directory
    pub story_graphs: String,
    /// Path to database files
    pub database: String,
    /// Path to assets directory
    pub assets: String,
    /// Path to maps directory
    #[serde(default = "default_maps_path")]
    pub maps: String,
    /// Path to modes directory
    #[serde(default = "default_modes_path")]
    pub modes: String,
    /// Path to scenarios directory
    #[serde(default = "default_scenarios_path")]
    pub scenarios: String,
}

fn default_maps_path() -> String { "maps".to_string() }
fn default_modes_path() -> String { "modes".to_string() }
fn default_scenarios_path() -> String { "scenarios".to_string() }

impl Default for ProjectPaths {
    fn default() -> Self {
        Self {
            scenes: "scenes".to_string(),
            story_graphs: "story_graphs".to_string(),
            database: "database".to_string(),
            assets: "assets".to_string(),
            maps: "maps".to_string(),
            modes: "modes".to_string(),
            scenarios: "scenarios".to_string(),
        }
    }
}

/// Autosave configuration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AutosaveSettings {
    /// Whether autosave is enabled
    pub enabled: bool,
    /// Autosave interval in seconds
    pub interval_seconds: u32,
    /// Maximum number of backup files to keep
    pub max_backups: u32,
}

impl Default for AutosaveSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            interval_seconds: 300, // 5 minutes
            max_backups: 10,
        }
    }
}

/// Editor snap settings for transform gizmos.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SnapSettings {
    /// Position snap increment
    pub position: f32,
    /// Rotation snap increment (degrees)
    pub rotation: f32,
    /// Scale snap increment
    pub scale: f32,
    /// Whether snapping is enabled
    pub enabled: bool,
}

impl Default for SnapSettings {
    fn default() -> Self {
        Self {
            position: 16.0, // Pixel grid
            rotation: 15.0, // 15 degree increments
            scale: 0.25,
            enabled: true,
        }
    }
}

/// Project-wide settings that affect both editor and runtime.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProjectSettings {
    /// Target platforms
    pub platforms: Vec<String>,
    /// Default resolution
    pub default_resolution: Size2i,
    /// Target frames per second
    pub target_fps: u32,
    /// Enable VSync
    pub vsync: bool,
    /// Enable pixel-perfect rendering
    pub pixel_perfect: bool,
    /// Input profile (JRPG, RTS, or Hybrid)
    pub input_profile: InputProfile,
    /// Localization settings
    pub localization: LocalizationSettings,
    /// Project file paths
    pub paths: ProjectPaths,
    /// Autosave settings
    pub autosave: AutosaveSettings,
}

impl Default for ProjectSettings {
    fn default() -> Self {
        Self {
            platforms: vec!["pc".to_string()],
            default_resolution: Size2i { width: 1280, height: 720 },
            target_fps: 60,
            vsync: true,
            pixel_perfect: true,
            input_profile: InputProfile::default(),
            localization: LocalizationSettings {
                languages: vec!["en".to_string()],
                default_language: "en".to_string(),
            },
            paths: ProjectPaths::default(),
            autosave: AutosaveSettings::default(),
        }
    }
}

/// Per-user editor preferences (not stored in project).
/// These are persisted to a user config file, NOT in the project.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EditorPreferences {
    /// Editor color theme
    pub theme: EditorTheme,
    /// UI scale factor
    pub ui_scale: f32,
    /// Font size in pixels
    pub font_size: u32,
    /// Grid cell size
    pub grid_size: Size2i,
    /// Snap settings
    pub snap: SnapSettings,
    /// Default gizmo mode
    pub default_gizmo_mode: GizmoMode,
    /// Custom keybindings (action -> key)
    pub keybindings: HashMap<String, String>,
    /// Layout preset
    pub layout_preset: LayoutPreset,
    /// Whether to automatically load the last opened project on startup
    #[serde(default = "default_load_last_project")]
    pub load_last_project: bool,
    /// List of recently opened project paths (most recent first)
    #[serde(default)]
    pub recent_projects: Vec<String>,
}

fn default_load_last_project() -> bool {
    true
}

impl Default for EditorPreferences {
    fn default() -> Self {
        Self {
            theme: EditorTheme::Dark,
            ui_scale: 1.0,
            font_size: 14,
            grid_size: Size2i { width: 32, height: 32 },
            snap: SnapSettings::default(),
            default_gizmo_mode: GizmoMode::Move,
            keybindings: HashMap::new(),
            layout_preset: LayoutPreset::JrpgMapping,
            load_last_project: true,
            recent_projects: Vec::new(),
        }
    }
}

impl EditorPreferences {
    /// Get the default preferences file path (~/.dj_engine/preferences.json)
    pub fn default_path() -> PathBuf {
        dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(".dj_engine")
            .join("preferences.json")
    }

    /// Load preferences from disk, or return default if not found
    pub fn load() -> Self {
        let path = Self::default_path();
        if path.exists() {
            match std::fs::read_to_string(&path) {
                Ok(contents) => {
                    match serde_json::from_str(&contents) {
                        Ok(prefs) => return prefs,
                        Err(e) => eprintln!("Failed to parse preferences: {}", e),
                    }
                }
                Err(e) => eprintln!("Failed to read preferences: {}", e),
            }
        }
        Self::default()
    }

    /// Save preferences to disk
    pub fn save(&self) -> Result<(), std::io::Error> {
        let path = Self::default_path();
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let json = serde_json::to_string_pretty(self)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        std::fs::write(&path, json)
    }

    /// Add a project to the recent projects list (moves to front if exists)
    pub fn add_recent_project(&mut self, path: impl Into<String>) {
        let path = path.into();
        // Remove if already exists
        self.recent_projects.retain(|p| p != &path);
        // Add to front
        self.recent_projects.insert(0, path);
        // Keep only last 10
        self.recent_projects.truncate(10);
    }

    /// Get the last opened project path, if any
    pub fn last_project(&self) -> Option<&str> {
        self.recent_projects.first().map(|s| s.as_str())
    }
}

/// Reference to a scene file.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SceneRef {
    /// Unique scene identifier
    pub id: String,
    /// Path to scene file (relative to project)
    pub path: String,
}

/// Reference to a story graph file.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StoryGraphRef {
    /// Unique story graph identifier
    pub id: String,
    /// Path to story graph file (relative to project)
    pub path: String,
}

/// Top-level project container.
///
/// A project encompasses all game content: scenes, story graphs,
/// databases, and asset references.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Project {
    /// Unique project identifier
    pub id: String,
    /// Human-readable project name
    pub name: String,
    /// Project version string
    pub version: String,
    /// Project-wide settings
    pub settings: ProjectSettings,
    /// Per-user editor preferences
    #[serde(default)]
    pub editor_preferences: EditorPreferences,
    /// List of scene references
    #[serde(default)]
    pub scenes: Vec<SceneRef>,
    /// List of story graph references
    #[serde(default)]
    pub story_graphs: Vec<StoryGraphRef>,
    /// Registry of maps (ID -> Path)
    #[serde(default)]
    pub maps: HashMap<String, String>,
    /// Registry of game modes (ID -> Path)
    #[serde(default)]
    pub modes: HashMap<String, String>,
    /// Registry of scenarios (ID -> Path)
    #[serde(default)]
    pub scenarios: HashMap<String, String>,
}

impl Default for Project {
    fn default() -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name: "New Project".to_string(),
            version: "0.1.0".to_string(),
            settings: ProjectSettings::default(),
            editor_preferences: EditorPreferences::default(),
            scenes: Vec::new(),
            story_graphs: Vec::new(),
            maps: HashMap::new(),
            modes: HashMap::new(),
            scenarios: HashMap::new(),
        }
    }
}

impl Project {
    /// Create a new project with the given name.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            ..Default::default()
        }
    }

    /// Add a scene reference to the project.
    pub fn add_scene(&mut self, id: impl Into<String>, path: impl Into<String>) {
        self.scenes.push(SceneRef {
            id: id.into(),
            path: path.into(),
        });
    }

    /// Add a story graph reference to the project.
    pub fn add_story_graph(&mut self, id: impl Into<String>, path: impl Into<String>) {
        self.story_graphs.push(StoryGraphRef {
            id: id.into(),
            path: path.into(),
        });
    }

    /// Find a scene reference by ID.
    pub fn find_scene(&self, id: &str) -> Option<&SceneRef> {
        self.scenes.iter().find(|s| s.id == id)
    }

    /// Find a story graph reference by ID.
    pub fn find_story_graph(&self, id: &str) -> Option<&StoryGraphRef> {
        self.story_graphs.iter().find(|s| s.id == id)
    }
}


/// Global engine settings (persisted per-user).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Resource)]
pub struct EngineSettings {
    pub draw_grid: bool,
    pub show_bounds: bool,
    pub log_scripts: bool,
    pub master_volume: f32,
    pub window_width: f32,
    pub window_height: f32,
    pub monitor_index: usize,
    pub window_mode_index: usize, // 0: Windowed, 1: Borderless, 2: Fullscreen
}

impl Default for EngineSettings {
    fn default() -> Self {
        Self {
            draw_grid: true,
            show_bounds: false,
            log_scripts: true,
            master_volume: 1.0,
            window_width: 1280.0,
            window_height: 720.0,
            monitor_index: 0,
            window_mode_index: 0,
        }
    }
}

impl EngineSettings {
    /// Get the default settings file path (~/.dj_engine/settings.json)
    pub fn default_path() -> PathBuf {
        dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(".dj_engine")
            .join("settings.json")
    }

    /// Load settings from disk, or return default if not found
    pub fn load() -> Self {
        let path = Self::default_path();
        if path.exists() {
            match std::fs::read_to_string(&path) {
                Ok(contents) => {
                    match serde_json::from_str(&contents) {
                        Ok(settings) => return settings,
                        Err(e) => eprintln!("Failed to parse settings: {}", e),
                    }
                }
                Err(e) => eprintln!("Failed to read settings: {}", e),
            }
        }
        Self::default()
    }

    /// Save settings to disk
    pub fn save(&self) -> Result<(), std::io::Error> {
        let path = Self::default_path();
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let json = serde_json::to_string_pretty(self)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        std::fs::write(&path, json)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_project_serialization() {
        let project = Project::new("Test Game");
        let json = serde_json::to_string_pretty(&project).unwrap();
        let deserialized: Project = serde_json::from_str(&json).unwrap();
        assert_eq!(project.name, deserialized.name);
    }

    #[test]
    fn test_add_scene() {
        let mut project = Project::new("Test");
        project.add_scene("level_01", "scenes/level_01.json");
        assert_eq!(project.scenes.len(), 1);
        assert!(project.find_scene("level_01").is_some());
    }
}
