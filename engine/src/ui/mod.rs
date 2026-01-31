use crate::story_graph::{events::{StoryFlowEvent, StoryInputEvent}, types::{GraphExecutor, ExecutionStatus}};
use bevy::prelude::*;

#[derive(Resource, Debug, Clone, Reflect)]
#[reflect(Resource)]
pub struct UiTheme {
    pub bg_color: Color,
    pub border_color: Color,
    pub text_primary: Color,
    pub text_secondary: Color,
    pub accent_color: Color,
    pub font_size_large: f32,
    pub font_size_medium: f32,
    pub font_size_small: f32,
}

impl Default for UiTheme {
    fn default() -> Self {
        Self {
            bg_color: Color::srgba(0.0, 0.05, 0.1, 0.9), // Deep Cyber Blue
            border_color: Color::srgb(0.0, 1.0, 0.8),    // Mint
            text_primary: Color::WHITE,
            text_secondary: Color::srgb(0.7, 0.8, 1.0),
            accent_color: Color::srgb(1.0, 0.0, 0.5),    // Neon Pink
            font_size_large: 24.0,
            font_size_medium: 18.0,
            font_size_small: 14.0,
        }
    }
}

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum GameMenuState {
    #[default]
    Playing,
    MainMenu,
    Paused,
    GameOver,
}

pub struct DJUiPlugin;

impl Plugin for DJUiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UiTheme>()
            .init_state::<GameMenuState>()
            .register_type::<UiTheme>()
            .add_systems(Update, (
                render_dialogue,
                render_choices,
                handle_ui_input,
            ));
    }
}

#[derive(Component)]
pub struct DialogueUiRoot;

#[derive(Component)]
pub struct DialogueText;

#[derive(Component)]
pub struct ChoiceUiRoot;

#[derive(Component)]
pub struct ChoiceButton(pub usize);

fn render_dialogue(
    mut commands: Commands,
    mut flow_events: MessageReader<StoryFlowEvent>,
    query: Query<Entity, With<DialogueUiRoot>>,
    theme: Res<UiTheme>,
) {
    for event in flow_events.read() {
        if let StoryFlowEvent::ShowDialogue { speaker, text, .. } = event {
            // Clear old UI
            for entity in &query {
                commands.entity(entity).despawn();
            }

            // Spawn new dialogue box
            commands
                .spawn((
                    Node {
                        position_type: PositionType::Absolute,
                        bottom: Val::Px(20.0),
                        left: Val::Percent(10.0),
                        right: Val::Percent(10.0),
                        height: Val::Px(150.0),
                        border: UiRect::all(Val::Px(2.0)),
                        padding: UiRect::all(Val::Px(15.0)),
                        flex_direction: FlexDirection::Column,
                        ..default()
                    },
                    BackgroundColor(theme.bg_color),
                    BorderColor::all(theme.border_color),
                    DialogueUiRoot,
                ))
                .with_children(|parent| {
                    // Speaker Name
                    parent.spawn((
                        Text::new(speaker),
                        TextFont {
                            font_size: 20.0,
                            ..default()
                        },
                        TextColor(theme.border_color),
                    ));

                    // Dialogue Text
                    parent.spawn((
                        Text::new(text),
                        TextFont {
                            font_size: theme.font_size_large,
                            ..default()
                        },
                        DialogueText,
                        TextColor(theme.text_primary),
                    ));
                });
        } else if matches!(event, StoryFlowEvent::GraphComplete) {
            for entity in &query {
                commands.entity(entity).despawn();
            }
        }
    }
}

fn render_choices(
    mut commands: Commands,
    mut flow_events: MessageReader<StoryFlowEvent>,
    query: Query<Entity, With<ChoiceUiRoot>>,
    theme: Res<UiTheme>,
) {
    for event in flow_events.read() {
        if let StoryFlowEvent::ShowChoices { options, .. } = event {
             // Clear old UI
             for entity in &query {
                commands.entity(entity).despawn();
            }

            commands.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    bottom: Val::Px(180.0),
                    left: Val::Percent(20.0),
                    right: Val::Percent(20.0),
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                ChoiceUiRoot,
            )).with_children(|parent| {
                for (i, opt) in options.iter().enumerate() {
                    parent.spawn((
                        Button,
                        Node {
                            width: Val::Percent(100.0),
                            height: Val::Px(40.0),
                            margin: UiRect::bottom(Val::Px(5.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        BackgroundColor(theme.bg_color.with_alpha(0.9)),
                        ChoiceButton(i),
                    )).with_children(|btn| {
                        btn.spawn((
                            Text::new(opt),
                            TextFont { font_size: theme.font_size_medium, ..default() },
                            TextColor(theme.text_primary),
                        ));
                    });
                }
            });
        }
    }
}

fn handle_ui_input(
    mut interaction_query: Query<
        (&Interaction, &ChoiceButton),
        (Changed<Interaction>, With<Button>),
    >,
    mut input_events: MessageWriter<StoryInputEvent>,
    mut commands: Commands,
    choice_root: Query<Entity, With<ChoiceUiRoot>>,
    _dialogue_root: Query<Entity, With<DialogueUiRoot>>,
    keys: Res<ButtonInput<KeyCode>>,
    executor: Res<GraphExecutor>,
) {
    // Handle Button Clicks
    for (interaction, choice) in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            input_events.write(StoryInputEvent::SelectChoice(choice.0));
            // Cleanup Choice UI
            for entity in &choice_root {
                commands.entity(entity).despawn();
            }
        }
    }

    // Handle Space/Enter to advance dialogue
    if keys.just_pressed(KeyCode::Space) || keys.just_pressed(KeyCode::Enter) {
        if executor.status == ExecutionStatus::WaitingForInput {
             // If Choice UI is NOT open, we can advance
             if choice_root.is_empty() {
                input_events.write(StoryInputEvent::Advance);
                // Simple feedback: could also clear dialogue here if it's the end
             }
        }
    }
}
