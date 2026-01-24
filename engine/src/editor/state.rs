use bevy::prelude::*;
use bevy_egui::egui::Color32;
use crate::data::story::StoryGraphData;
use crate::data::map::MapAsset;
use crate::data::scenario::ScenarioData;

pub const COLOR_PRIMARY: Color32 = Color32::from_rgb(0, 255, 204); // Cyberpunk Mint
pub const COLOR_SECONDARY: Color32 = Color32::from_rgb(255, 175, 200); // Pale Rose
pub const COLOR_BG: Color32 = Color32::from_rgb(15, 15, 20);

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum EditorState {
    #[default]
    Editor,
    Playing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SidePanelTab {
    #[default]
    Hierarchy,
    Palette,
    Assets,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum EditorView {
    #[default]
    Core,           // Workspace Dashboard
    FeatureGrid,    // Ecosystem toggle grid
    Timeline,       // DAW-style Logic Timeline
    MapEditor,      // Edit static geometry (MapAsset)
    ScenarioEditor, // Edit dynamic entities (ScenarioData)
    StoryGraph,
    Campaign,
    Controls,
    Settings,
    Phases,
    Play,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Commit {
    pub id: String,
    pub message: String,
    pub status: CommitStatus,
    pub position: bevy::math::Vec2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommitStatus {
    Passed,
    Failed,
    Pending,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Branch {
    pub id: String,
    pub name: String,
    pub color: Color32,
    pub active_view: EditorView,
    pub active_tab: SidePanelTab,
    pub history: Vec<Commit>,
    pub node_overrides: std::collections::HashMap<String, bool>, // feature_id -> enabled
}

// ============== FEATURE GRID ==============

#[derive(Debug, Clone)]
pub struct Ecosystem {
    pub id: String,
    pub name: String,
    pub icon: String,
    pub color: Color32,
    pub nodes: Vec<FeatureNode>,
    pub position: Vec2,
}

#[derive(Debug, Clone)]
pub struct FeatureNode {
    pub id: String,
    pub name: String,
    pub icon: String,
    pub description: String,
}

#[derive(Resource)]
pub struct FeatureGrid {
    pub ecosystems: Vec<Ecosystem>,
}

impl Default for FeatureGrid {
    fn default() -> Self {
        Self {
            ecosystems: vec![
                Ecosystem {
                    id: "abilities".into(),
                    name: "Abilities".into(),
                    icon: "⚡".into(),
                    color: Color32::from_rgb(255, 100, 100),
                    position: Vec2::new(-200.0, -100.0),
                    nodes: vec![
                        FeatureNode { id: "fireball".into(), name: "Fireball".into(), icon: "🔥".into(), description: "Launch a ball of fire".into() },
                        FeatureNode { id: "ice_spear".into(), name: "Ice Spear".into(), icon: "❄️".into(), description: "Throw a frozen spear".into() },
                        FeatureNode { id: "lightning".into(), name: "Lightning".into(), icon: "⚡".into(), description: "Call down lightning".into() },
                    ],
                },
                Ecosystem {
                    id: "items".into(),
                    name: "Items".into(),
                    icon: "📦".into(),
                    color: Color32::from_rgb(100, 200, 100),
                    position: Vec2::new(200.0, -100.0),
                    nodes: vec![
                        FeatureNode { id: "potion_hp".into(), name: "Health Potion".into(), icon: "❤️".into(), description: "Restores HP".into() },
                        FeatureNode { id: "potion_mp".into(), name: "Mana Potion".into(), icon: "💙".into(), description: "Restores MP".into() },
                    ],
                },
                Ecosystem {
                    id: "physics".into(),
                    name: "Physics".into(),
                    icon: "⚙️".into(),
                    color: Color32::from_rgb(100, 150, 255),
                    position: Vec2::new(0.0, 150.0),
                    nodes: vec![
                        FeatureNode { id: "gravity".into(), name: "Gravity".into(), icon: "🌍".into(), description: "World gravity".into() },
                        FeatureNode { id: "collision".into(), name: "Collision".into(), icon: "💥".into(), description: "Collision detection".into() },
                    ],
                },
            ],
        }
    }
}

#[derive(Resource, Default)]
pub struct ProjectMetadata {
    pub name: String,
    pub path: Option<std::path::PathBuf>,
}

#[derive(Resource, Default)]
pub struct ActiveStoryGraph(pub StoryGraphData);

#[derive(Resource, Default)]
pub struct ActiveMap(pub MapAsset);

#[derive(Resource, Default)]
pub struct ActiveScenario(pub ScenarioData);


#[derive(Resource, Default)]
pub struct EditorUiState {
    pub global_view: EditorView,
    pub active_branches: Vec<Branch>,
    pub active_branch_idx: usize,
    pub selected_entities: bevy_inspector_egui::bevy_inspector::hierarchy::SelectedEntities,
    pub asset_search_query: String,
    pub selected_palette_item: Option<String>,
    pub console_open: bool,
    pub console_input: String,
    pub dragged_node_id: Option<String>,
    pub connection_start_id: Option<String>,
    pub selected_node_id: Option<String>,
}

impl EditorUiState {
    pub fn new() -> Self {
        Self {
            global_view: EditorView::Core,
            active_branches: vec![Branch {
                id: "main".to_string(),
                name: "Main Branch".to_string(),
                color: COLOR_PRIMARY,
                active_view: EditorView::MapEditor,
                active_tab: SidePanelTab::Hierarchy,
                history: vec![
                    Commit {
                        id: "initial".to_string(),
                        message: "Initial Commit".to_string(),
                        status: CommitStatus::Passed,
                        position: Vec2::new(0.0, 0.0),
                    }
                ],
                node_overrides: std::collections::HashMap::new(),
            }],
            active_branch_idx: 0,
            ..Default::default()
        }
    }

    pub fn current_branch(&self) -> Option<&Branch> {
        self.active_branches.get(self.active_branch_idx)
    }

    pub fn current_branch_mut(&mut self) -> Option<&mut Branch> {
        self.active_branches.get_mut(self.active_branch_idx)
    }
}

/// Dynamic settings for the engine, controlled via UI.
#[derive(Resource, Debug, Clone)]
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
            log_scripts: false,
            master_volume: 0.8,
            window_width: 1920.0,
            window_height: 1080.0,
            monitor_index: 0,
            window_mode_index: 0,
        }
    }
}

#[derive(Resource)]
pub struct AutomatedTestActive {
    pub timer: Timer,
    pub step: usize,
}

/// Marker component for entities that should show up in the game scene hierarchy.
#[derive(Component, Default)]
pub struct LogicalEntity;
