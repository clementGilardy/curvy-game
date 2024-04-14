use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};

use crate::{despawn_screen, GameState};
use crate::game::player::Player;

pub fn game_plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Game), game_setup)
        .add_systems(Update, (game, mouvement, gameover).run_if(in_state(GameState::Game)))
        .add_systems(OnExit(GameState::Game), despawn_screen::<OnGameScreen>);
}

// Tag component used to tag entities added on the game screen
#[derive(Component)]
struct OnGameScreen;

#[derive(Resource)]
struct Game {
    player1: Player,
    state: GameState,
}

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
    let shape = Mesh2dHandle(meshes.add(Circle { radius: 10.0 }));
    let color = Color::hsl(360., 0.95, 0.7);
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: shape,
            material: materials.add(color),
            transform: Transform::from_xyz(
                100.0,
                0.0,
                0.0,
            ),
            ..default()
        },
        Direction::Up
    ));
}

fn game(windows: Query<&Window>, time: Res<Time>, mut shape_position: Query<(&mut Direction, &mut Transform)>) {
    for (mut logo, mut transform) in &mut shape_position {
        let window = windows.single();
        let height = window.height();
        let width = window.width();
        match *logo {
            Direction::Up => transform.translation.y += 150. * time.delta_seconds(),
            Direction::Down => transform.translation.y -= 150. * time.delta_seconds(),
            Direction::Right => transform.translation.x += 150. * time.delta_seconds(),
            Direction::Left => transform.translation.x -= 150. * time.delta_seconds(),
            Direction::Stop => {
                transform.translation.x += 0.;
                transform.translation.y += 0.
            }
        }
        behaviour_on_y(height, &transform, &mut logo);
        behaviour_on_x(width, &transform, &mut logo);
    }
}

fn mouvement(keyboard_input: Res<ButtonInput<KeyCode>>, mut position: Query<&mut Direction>) {
    for mut logo in &mut position {
        if keyboard_input.just_pressed(KeyCode::KeyA) {
            match *logo {
                Direction::Right => {}
                _ => {
                    *logo = Direction::Left;
                }
            }
        } else if keyboard_input.just_pressed(KeyCode::KeyD) {
            match *logo {
                Direction::Left => {}
                _ => {
                    *logo = Direction::Right;
                }
            }
        } else if keyboard_input.just_pressed(KeyCode::KeyW) {
            match *logo {
                Direction::Down => {}
                _ => {
                    *logo = Direction::Up;
                }
            }
        } else if keyboard_input.just_pressed(KeyCode::KeyS) {
            match *logo {
                Direction::Up => {}
                _ => {
                    *logo = Direction::Down;
                }
            }
        }
    }
}

fn behaviour_on_x(width: f32, transform: &Mut<Transform>, logo: &mut Direction) {
    if transform.translation.x > (width / 2.) {
        *logo = Direction::Stop;
    } else if transform.translation.x < -(width / 2.) {
        *logo = Direction::Stop
    }
}

fn behaviour_on_y(height: f32, transform: &Mut<Transform>, logo: &mut Direction) {
    if transform.translation.y > (height / 2.) {
        *logo = Direction::Stop;
    } else if transform.translation.y < -(height / 2.) {
        *logo = Direction::Stop
    }
}

fn gameover(
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