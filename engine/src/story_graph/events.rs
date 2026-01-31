use bevy::prelude::*;

/// Events sent FROM the Executor TO the UI/Game
#[derive(Message, Debug, Clone)]
pub enum StoryFlowEvent {
    ShowDialogue {
        speaker: String,
        text: String,
        portrait: Option<String>,
    },
    ShowChoices {
        prompt: String,
        options: Vec<String>,
    }, // Only send text to UI
    CameraControl {
        preset_id: Option<String>,
        position: Vec3,
        zoom: f32,
        duration: f32,
    },
    TimeControl {
        time_scale: f32,
        pause: bool,
    },
    StartBattle {
        enemy_id: String,
    },
    GraphComplete,
}

/// Events sent FROM the UI/Game TO the Executor
#[derive(Message, Debug, Clone)]
pub enum StoryInputEvent {
    Advance,
    SelectChoice(usize),
    FinishBattle { won: bool },
}

/// Generic story event
#[derive(Message)]
pub struct StoryEvent {
    pub id: String,
    pub payload: String,
}

