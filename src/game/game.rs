use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};

use crate::{despawn_screen, GameState};
use crate::game::snake::{Corpse, Position, Snake};

pub fn game_plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Game), game_setup)
        .add_systems(Update, (game_direction, mouvement, trace, end_game).run_if(in_state(GameState::Game)))
        .add_systems(OnExit(GameState::Game), despawn_screen::<OnGameScreen>);
}

// Tag component used to tag entities added on the game screen
#[derive(Component)]
pub struct OnGameScreen;

#[derive(Resource)]
struct Game;

#[derive(Component, Debug)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
    Stop,
}

fn game_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let position = Position {
        x: 100.,
        y: 20.,
    };
    let mut positions = Vec::new();
    positions.push(position.clone());
    let color = Color::hsl(360., 0.95, 0.7);
    let material = materials.add(color);
    let transform = Transform::from_xyz(position.x, position.y, 0.0);
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Circle { radius: 10.0 })),
            material,
            transform,
            ..Default::default()
        },
        Snake,
        Corpse {
            positions
        },
        Direction::Up,
        OnGameScreen
    ));
}

fn game_direction(
    windows: Query<&Window>,
    time: Res<Time>,
    mut query: Query<(&mut Direction, &mut Corpse), With<Snake>>,
) {
    let (mut direction, mut corpse) = query.single_mut();
    let window = windows.single();
    let height = window.height();
    let width = window.width();

    // Calcul de la nouvelle position en fonction de la direction
    let new_position = match *direction {
        Direction::Up | Direction::Down => Position {
            y: corpse.positions.last().map_or(0.0, |last_pos| {
                last_pos.y + match *direction {
                    Direction::Up => 150. * time.delta_seconds(),
                    Direction::Down => -150. * time.delta_seconds(),
                    _ => 0.0,
                }
            }),
            x: corpse.positions.last().map_or(0.0, |last_pos| last_pos.x),
        },
        Direction::Right | Direction::Left => Position {
            x: corpse.positions.last().map_or(0.0, |last_pos| {
                last_pos.x + match *direction {
                    Direction::Right => 150. * time.delta_seconds(),
                    Direction::Left => -150. * time.delta_seconds(),
                    _ => 0.0,
                }
            }),
            y: corpse.positions.last().map_or(0.0, |last_pos| last_pos.y),
        },
        Direction::Stop => Position { x: 0.0, y: 0.0 },
    };

    // Ajout de la nouvelle position dans la liste des positions du corpse
    corpse.positions.push(new_position);

    // Appel des fonctions de comportement sur l'axe y et l'axe x
    behaviour_on_y(height, &corpse.positions.last().unwrap(), &mut direction);
    behaviour_on_x(width, &corpse.positions.last().unwrap(), &mut direction);
}

fn trace(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>, mut meshes: ResMut<Assets<Mesh>>, mut query: Query<&mut Corpse, With<Snake>>) {
    let corpse = query.single_mut();
    let last_position = corpse.positions.last().unwrap();
    let color = Color::hsl(360., 0.95, 0.7);
    let material = materials.add(color);
    let transform = Transform::from_xyz(last_position.x, last_position.y, 0.0);
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Circle { radius: 10.0 })),
            material,
            transform,
            ..Default::default()
        },
    ));
}

fn mouvement(keyboard_input: Res<ButtonInput<KeyCode>>, mut position: Query<&mut Direction>) {
    for mut direction in &mut position {
        if keyboard_input.just_pressed(KeyCode::KeyA) {
            match *direction {
                Direction::Right => {}
                _ => {
                    *direction = Direction::Left;
                }
            }
        } else if keyboard_input.just_pressed(KeyCode::KeyD) {
            match *direction {
                Direction::Left => {}
                _ => {
                    *direction = Direction::Right;
                }
            }
        } else if keyboard_input.just_pressed(KeyCode::KeyW) {
            match *direction {
                Direction::Down => {}
                _ => {
                    *direction = Direction::Up;
                }
            }
        } else if keyboard_input.just_pressed(KeyCode::KeyS) {
            match *direction {
                Direction::Up => {}
                _ => {
                    *direction = Direction::Down;
                }
            }
        }
    }
}

fn behaviour_on_x(width: f32, position: &Position, direction: &mut Direction) {
    if position.x > (width / 2.) {
        *direction = Direction::Stop;
    } else if position.x < -(width / 2.) {
        *direction = Direction::Stop
    }
}

fn behaviour_on_y(height: f32, position: &Position, direction: &mut Direction) {
    if position.y > (height / 2.) {
        *direction = Direction::Stop;
    } else if position.y < -(height / 2.) {
        *direction = Direction::Stop
    }
}

fn end_game(
    mut game_state: ResMut<NextState<GameState>>,
    query: Query<&Direction>,
) {
    for direction in query.iter() {
        match direction {
            Direction::Stop => {
                print!("game over");
                game_state.set(GameState::GameOver);
            }
            _ => {}
        }
    }
}