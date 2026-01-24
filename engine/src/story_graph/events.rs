use bevy::prelude::*;

/// Events sent FROM the Executor TO the UI/Game
#[derive(Event, Debug, Clone)]
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
    GraphComplete,
}

/// Events sent FROM the UI/Game TO the Executor
#[derive(Event, Debug, Clone)]
pub enum StoryInputEvent {
    Advance,
    SelectChoice(usize),
}

/// Generic story event
#[derive(Event)]
pub struct StoryEvent {
    pub id: String,
    pub payload: String,
}
