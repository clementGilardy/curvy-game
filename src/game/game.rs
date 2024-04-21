use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};

use crate::{despawn_screen, GameState};
use crate::game::snake::{Direction, Position, Snake};

pub fn game_plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Game), game_setup)
        .add_systems(
            Update,
            (game_direction, mouvement, trace_snake_corpse, queue_collision, end_game).run_if(in_state(GameState::Game)),
        )
        .add_systems(OnExit(GameState::Game), despawn_screen::<OnGameScreen>);
}

// Tag component used to tag entities added on the game screen
#[derive(Component)]
pub struct OnGameScreen;

#[derive(Resource)]
struct Game;

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
        Snake {
            positions,
            direction_before_pause: Direction::Up,
        },
        Direction::Up,
        OnGameScreen
    ));
}

fn game_direction(
    windows: Query<&Window>,
    time: Res<Time>,
    mut query: Query<(&mut Direction, &mut Snake), With<Snake>>,
) {
    let (mut direction, mut snake) = query.single_mut();
    if *direction != Direction::Pause {
        let window = windows.single();
        let height = window.height();
        let width = window.width();
        snake.direction_before_pause = direction.clone();
        // Calcul de la nouvelle position en fonction de la direction
        let new_position = match *direction {
            Direction::Up | Direction::Down => Position {
                y: snake.positions.last().map_or(0.0, |last_pos| {
                    last_pos.y + match *direction {
                        Direction::Up => 150. * time.delta_seconds(),
                        Direction::Down => -150. * time.delta_seconds(),
                        _ => 0.0,
                    }
                }),
                x: snake.positions.last().map_or(0.0, |last_pos| last_pos.x),
            },
            Direction::Right | Direction::Left => Position {
                x: snake.positions.last().map_or(0.0, |last_pos| {
                    last_pos.x + match *direction {
                        Direction::Right => 150. * time.delta_seconds(),
                        Direction::Left => -150. * time.delta_seconds(),
                        _ => 0.0,
                    }
                }),
                y: snake.positions.last().map_or(0.0, |last_pos| last_pos.y),
            },
            Direction::Stop => Position { x: 0.0, y: 0.0 },
            Direction::Pause => snake.positions.last().unwrap().clone()
        };

        // Ajout de la nouvelle position dans la liste des positions du snake
        snake.positions.push(new_position);

        // Appel des fonctions de comportement sur l'axe y et l'axe x
        behaviour_on_y(height, &snake.positions.last().unwrap(), &mut direction);
        behaviour_on_x(width, &snake.positions.last().unwrap(), &mut direction);
    }
}

fn trace_snake_corpse(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>, mut meshes: ResMut<Assets<Mesh>>, mut query: Query<(&mut Snake, &mut Direction), With<Snake>>) {
    let (snake, direction) = query.single_mut();
    if *direction != Direction::Pause {
        let last_position = snake.positions.last().unwrap();
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
            OnGameScreen
        ));
    }
}

fn mouvement(keyboard_input: Res<ButtonInput<KeyCode>>, mut query: Query<(&mut Direction, &mut Snake), With<Snake>>) {
    let (mut direction, snake) = query.single_mut();
    if keyboard_input.just_pressed(KeyCode::Escape) && *direction != Direction::Pause {
        *direction = Direction::Pause;
    } else if keyboard_input.just_pressed(KeyCode::Escape) && *direction == Direction::Pause {
        *direction = snake.direction_before_pause.clone();
    } else if let Some(new_direction) = match keyboard_input.just_pressed(KeyCode::KeyA) {
        true if *direction != Direction::Right => Some(Direction::Left),
        _ => None,
    } {
        *direction = new_direction;
    } else if let Some(new_direction) = match keyboard_input.just_pressed(KeyCode::KeyD) {
        true if *direction != Direction::Left => Some(Direction::Right),
        _ => None,
    } {
        *direction = new_direction;
    } else if let Some(new_direction) = match keyboard_input.just_pressed(KeyCode::KeyW) {
        true if *direction != Direction::Down => Some(Direction::Up),
        _ => None,
    } {
        *direction = new_direction;
    } else if let Some(new_direction) = match keyboard_input.just_pressed(KeyCode::KeyS) {
        true if *direction != Direction::Up => Some(Direction::Down),
        _ => None,
    } {
        *direction = new_direction;
    }
}

fn queue_collision(mut query: Query<(&mut Direction, &mut Snake), With<Snake>>) {
    let (mut direction, snake) = query.single_mut();
    if snake.is_last_position_duplicate() {
        *direction = Direction::Stop;
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
) -> () {
    let direction = query.single();
    match direction {
        Direction::Stop => {
            print!("game over");
            game_state.set(GameState::GameOver);
        }
        _ => {}
    }
}