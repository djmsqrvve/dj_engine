use bevy::prelude::*;
use std::collections::{HashMap, VecDeque};

/// Distinct phases of the game engine lifecycle.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, States, Reflect)]
pub enum GamePhase {
    #[default]
    Loading,
    Initialization,
    MainMenu,
    Gameplay,
    Paused,
    Shutdown,
}

/// Status of a tracked task.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Reflect)]
pub enum TaskStatus {
    Pending,
    InProgress,
    Completed,
    Failed,
}

/// Resource to manage game phases and track readiness.
#[derive(Resource, Default, Reflect)]
#[reflect(Resource)]
pub struct PhaseManager {
    pub current_phase: GamePhase,
    /// Ordered list of phases for visualization
    pub phase_order: Vec<GamePhase>,
    /// Tasks required for the current phase to complete (Task Name -> Status)
    pub pending_tasks: HashMap<String, TaskStatus>,
    /// Log of phase transitions and validation events
    pub event_log: VecDeque<String>,
}

impl PhaseManager {
    pub fn new() -> Self {
        Self {
            current_phase: GamePhase::Loading,
            phase_order: vec![
                GamePhase::Loading,
                GamePhase::Initialization,
                GamePhase::MainMenu,
                GamePhase::Gameplay,
                GamePhase::Paused,
                GamePhase::Shutdown,
            ],
            pending_tasks: HashMap::new(),
            event_log: VecDeque::new(),
        }
    }

    pub fn set_phase(&mut self, phase: GamePhase) {
        if self.current_phase != phase {
            let msg = format!("Transition: {:?} -> {:?}", self.current_phase, phase);
            self.log(msg);
            self.current_phase = phase;
            // Clear tasks on transition? Or keep history? 
            // For now, clear to track new phase requirements.
            self.pending_tasks.clear(); 
        }
    }

    pub fn register_task(&mut self, task_name: impl Into<String>) {
        self.pending_tasks.insert(task_name.into(), TaskStatus::Pending);
    }

    pub fn update_task(&mut self, task_name: &str, status: TaskStatus) {
        if let Some(s) = self.pending_tasks.get_mut(task_name) {
            *s = status;
            if status == TaskStatus::Failed {
                self.log(format!("Task Failed: {}", task_name));
            } else if status == TaskStatus::Completed {
                self.log(format!("Task Completed: {}", task_name));
            }
        }
    }

    pub fn log(&mut self, message: impl Into<String>) {
        if self.event_log.len() >= 50 {
            self.event_log.pop_front();
        }
        self.event_log.push_back(message.into());
    }
}

/// Event triggered when the game phase changes.
#[derive(Event, Debug, Clone)]
pub struct PhaseChangeEvent {
    pub prev: GamePhase,
    pub next: GamePhase,
}

pub struct GamePhasePlugin;

impl Plugin for GamePhasePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GamePhase>()
           .insert_resource(PhaseManager::new())
           .register_type::<GamePhase>()
           .register_type::<TaskStatus>()
           .register_type::<PhaseManager>()
           .add_event::<PhaseChangeEvent>()
           .add_systems(OnEnter(GamePhase::Loading), (
               setup_loading_tasks,
               validate_assets_system.after(setup_loading_tasks)
           ))
           .add_systems(Update, sync_bevy_state);
    }
}

fn setup_loading_tasks(mut manager: ResMut<PhaseManager>) {
    manager.log("Engine Started. Phase: Loading");
    manager.register_task("Core Assets");
    manager.register_task("Renderer Init");
    manager.register_task("Audio System");
}

fn validate_assets_system(mut manager: ResMut<PhaseManager>) {
    // simulate some work or check real paths
    let music_path = std::path::Path::new("assets/music/overworld_theme.mid");
    
    if music_path.exists() {
        manager.update_task("Core Assets", TaskStatus::Completed);
        manager.log("Asset validation passed.");
    } else {
        manager.update_task("Core Assets", TaskStatus::Failed);
        manager.log("MISSING ASSETS: Run './dj gen' to generate placeholders.");
    }

    // Auto-complete others for prototype
    manager.update_task("Renderer Init", TaskStatus::Completed);
    manager.update_task("Audio System", TaskStatus::Completed);
    
    // Auto-transition if all good
    // In a real game, this would wait for async asset loading
    // manager.set_phase(GamePhase::Initialization);
}

fn sync_bevy_state(
    mut next_state: ResMut<NextState<GamePhase>>,
    manager: Res<PhaseManager>,
    current_state: Res<State<GamePhase>>,
) {
    if manager.current_phase != *current_state.get() {
        next_state.set(manager.current_phase);
    }
}
