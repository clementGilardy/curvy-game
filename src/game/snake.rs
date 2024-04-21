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

#[derive(Clone, PartialEq, Copy, Debug)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl Snake {
    pub fn is_last_position_duplicate(&self) -> bool {
        if let Some(last_position) = self.positions.last() {
            for pos in self.positions.iter().rev().skip(1) {
                if (pos.y - last_position.y).abs() <= 2. && (pos.x - last_position.x).abs() <= 2. {
                    return true;
                }
            }
        }
        false
    }
}