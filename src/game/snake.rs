use bevy::prelude::Component;

#[derive(Component)]
pub struct Snake;

#[derive(Component)]
pub struct Corpse {
    pub positions: Vec<Position>,
}

#[derive(Clone)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}