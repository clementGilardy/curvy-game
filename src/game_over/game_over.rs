use bevy::app::App;
use bevy::prelude::{AlignItems, BuildChildren, Commands, Component, default, FlexDirection, JustifyContent, NodeBundle, OnEnter, OnExit, Style, TextBundle, TextStyle, UiRect, Val};
use bevy::prelude::Color::Rgba;

use crate::{despawn_screen, GameState};
use crate::game::game::OnGameScreen;

// Tag component used to tag entities added on the splash screen
#[derive(Component)]
struct OnGameOverScreen;

pub fn game_over_plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::GameOver), (despawn_screen::<OnGameScreen>, game_over_setup))
        .add_systems(OnExit(GameState::GameOver), despawn_screen::<OnGameOverScreen>);
}

pub fn game_over_setup(mut commands: Commands) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            OnGameOverScreen,
        )).with_children(|parent| {
        parent
            .spawn(NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            })
            .with_children(|parent| {
                // Display the game name
                parent.spawn(
                    TextBundle::from_section(
                        "Game Over",
                        TextStyle {
                            font_size: 80.0,
                            color: Rgba { alpha: 1.0, red: 1.0, blue: 1.0, green: 1.0 },
                            ..default()
                        },
                    )
                        .with_style(Style {
                            margin: UiRect::all(Val::Px(50.0)),
                            ..default()
                        }),
                );
            });
    });
}
