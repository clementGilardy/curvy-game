use bevy::prelude::Component;

#[derive(Component)]
pub struct Snake {
    pub positions: Vec<Position>,
    pub direction_before_pause: Direction,
}

#[derive(Component, Debug, PartialEq, Clone)]
pub enum Direction {
    Up,
    Down,
    Right,
    Left,
    Pause,
    Stop,
}

#[derive(Clone)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}