use bevy::prelude::*;

#[derive(Component)]
pub struct HUD;

pub fn setup_hud(mut commands: Commands) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    position_type: PositionType::Absolute,
                    ..default()
                },
                ..default()
            },
            HUD,
        ))
        .with_children(|parent| {
            parent.spawn(
                TextBundle::from_section(
                    "Bevy 2D Rendering Sandbox",
                    TextStyle {
                        font_size: 24.0,
                        color: Color::srgb(0.0, 1.0, 0.5),
                        ..default()
                    },
                )
                .with_style(Style {
                    position_type: PositionType::Absolute,
                    left: Val::Px(20.0),
                    top: Val::Px(20.0),
                    ..default()
                }),
            );
        });
}
