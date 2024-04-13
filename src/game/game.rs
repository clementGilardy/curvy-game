use bevy::prelude::*;
use bevy::render::render_resource::ShaderType;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};

use crate::{despawn_screen, GameState};

// This plugin will contain the game. In this case, it's just be a screen that will
// display the current settings for 5 seconds before returning to the menu
pub fn game_plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Game), game_setup)
        .add_systems(Update, game)
        .add_systems(OnExit(GameState::Game), despawn_screen::<OnGameScreen>);
}

// Tag component used to tag entities added on the game screen
#[derive(Component)]
struct OnGameScreen;

#[derive(Component)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
    Stop
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
            Direction::Right => transform.translation.x -= 150. * time.delta_seconds(),
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