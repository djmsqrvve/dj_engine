use bevy::prelude::*;

#[derive(Event, Debug, Clone)]
pub enum AnimationCommand {
    SetExpression { target_id: String, expression: String },
    PlayAnimation { target_id: String, clip: String },
}
