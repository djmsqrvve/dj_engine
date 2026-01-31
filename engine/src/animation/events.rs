use bevy::prelude::*;

#[derive(Message, Debug, Clone)]
pub enum AnimationCommand {
    SetExpression { target_id: String, expression: String },
    PlayAnimation { target_id: String, clip: String },
}
