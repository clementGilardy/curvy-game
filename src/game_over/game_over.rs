use bevy::app::{App, AppExit, Update};
use bevy::prelude::{AlignItems, BuildChildren, Button, ButtonBundle, Changed, Commands, Component, default, EventWriter, FlexDirection, in_state, Interaction, IntoSystemConfigs, JustifyContent, NextState, NodeBundle, OnEnter, OnExit, Query, ResMut, Style, TextBundle, TextStyle, UiImage, UiRect, Val, With};
use bevy::prelude::Color::Rgba;
use bevy::ui::AlignContent;

use crate::{despawn_screen, GameState};
use crate::game::game::OnGameScreen;

// Tag component used to tag entities added on the splash screen
#[derive(Component)]
pub struct OnGameOverScreen;

#[derive(Component)]
enum GameOverMenuButtonAction {
    Replay,
    BackToMainMenu,
    Quit,
}

pub fn game_over_plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::GameOver), (despawn_screen::<OnGameScreen>, game_over_setup))
        .add_systems(Update, menu_action.run_if(in_state(GameState::GameOver)))
        .add_systems(OnExit(GameState::GameOver), despawn_screen::<OnGameOverScreen>)
        .add_systems(Update, (menu_action, crate::menu::menu::button_system).run_if(in_state(GameState::GameOver)));
}

pub fn game_over_setup(mut commands: Commands) {
    let button_style = Style {
        width: Val::Px(200.0),
        height: Val::Px(65.0),
        margin: UiRect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        align_content: AlignContent::Center,
        ..default()
    };

    let button_text_style = TextStyle {
        font_size: 40.0,
        color: Rgba { alpha: 1.0, red: 0.0, blue: 0.0, green: 0.0 },
        ..default()
    };
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
            }).with_children(|parent| {
            for (action, text) in [
                (GameOverMenuButtonAction::Replay, "Rejouer"),
                (GameOverMenuButtonAction::BackToMainMenu, "Retour au menu"),
                (GameOverMenuButtonAction::Quit, "Quitter"),
            ] {
                parent
                    .spawn((
                        ButtonBundle {
                            style: button_style.clone(),
                            image: UiImage::default(),
                            ..default()
                        },
                        action,
                    ))
                    .with_children(|parent| {
                        parent.spawn(TextBundle::from_section(
                            text,
                            button_text_style.clone(),
                        ));
                    });
            }
        });
    });
}

fn menu_action(interaction_query: Query<
    (&Interaction, &GameOverMenuButtonAction),
    (Changed<Interaction>, With<Button>),
>, mut app_exit_events: EventWriter<AppExit>, mut menu_state: ResMut<NextState<crate::menu::menu::MenuState>>, mut game_state: ResMut<NextState<GameState>>, ) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button_action {
                GameOverMenuButtonAction::Replay => {
                    game_state.set(GameState::Game);
                    menu_state.set(crate::menu::menu::MenuState::Disabled);
                }
                GameOverMenuButtonAction::BackToMainMenu => {
                    game_state.set(GameState::Menu);
                    menu_state.set(crate::menu::menu::MenuState::Main);
                }
                GameOverMenuButtonAction::Quit => {
                    app_exit_events.send(AppExit);
                }
            }
        }
    }
}
