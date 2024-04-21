use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;

use game::game::game_plugin;
use game_over::game_over::game_over_plugin;
use menu::menu::menu_plugin;
use splash::splash::splash_plugin;

mod splash;
mod game;
mod menu;
pub mod game_over;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GameState {
    #[default]
    Splash,
    Menu,
    Game,
    GameOver,
}

// One of the two settings that can be set through the menu. It will be a resource in the app
#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
enum DisplayQuality {
    Low,
    Medium,
    High,
}

// One of the two settings that can be set through the menu. It will be a resource in the app
#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
struct Volume(u32);

fn main() {
    App::new()
        .add_plugins(default_plugin())
        .insert_resource(DisplayQuality::Medium)
        .insert_resource(Volume(7))
        .init_state::<GameState>()
        .add_systems(Startup, setup)
        .add_plugins((splash_plugin, menu_plugin, game_plugin, game_over_plugin))
        .run();
}

fn default_plugin() -> PluginGroupBuilder {
    DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Curvy game".to_string(),
            ..Default::default()
        }),
        ..Default::default()
    })
}

fn setup(mut commands: Commands) -> () {
    commands.spawn(Camera2dBundle::default());
}

fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}
