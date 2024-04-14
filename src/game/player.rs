use bevy::prelude::Entity;

use crate::game::effect::{Bonus, CurvEffect};

#[derive(Default)]
pub struct Player {
    entity: Option<Entity>,
    x: usize,
    y: usize,
    effects: Box<dyn CurvEffect>,
}

impl Player {
    pub fn new() -> Self {
        Player {
            entity: None,
            x: 0,
            y: 0,
            effects: Box::new(Bonus::new()),
        }
    }
}
