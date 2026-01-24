//! Node-based story execution system.
//!
//! Replaces linear dialogue queues with a directed graph of nodes.
//! Supports branching logic, events, and complex narrative flow.

pub mod executor;

pub mod types;

pub mod events;

use bevy::prelude::*;

pub use types::*;

pub use events::*;

pub struct StoryGraphPlugin;

impl Plugin for StoryGraphPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<StoryGraph>()
            .register_type::<StoryNode>()
            .register_type::<GraphChoice>()
            .register_type::<StoryFlags>()
            .register_type::<ExecutionStatus>()
            .register_type::<GraphExecutor>()
            .init_resource::<GraphExecutor>()
            .init_resource::<StoryFlags>()
            .init_resource::<StoryGraphLibrary>()
            .add_event::<StoryFlowEvent>()
            .add_event::<StoryInputEvent>()
            .add_event::<StoryEvent>()
            .add_event::<executor::StoryActionEvent>()
            .add_systems(Update, executor::execute_graph);
    }
}
