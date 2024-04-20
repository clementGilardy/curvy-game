use bevy::sprite::{ColorMaterial, MaterialMesh2dBundle};

use crate::game::effect::CurvEffect;

#[derive(Default)]
pub struct Player {
    id: u8,
    pub x: f32,
    pub y: f32,
    snake: Option<MaterialMesh2dBundle<ColorMaterial>>,
    effects: Option<Vec<Box<dyn CurvEffect>>>,
}

impl Player {
    pub fn new() -> Self {
        static mut NEXT_ID: u8 = 0;
        unsafe {
            NEXT_ID += 1;
            Player {
                id: NEXT_ID,
                x: 0.,
                y: 0.,
                snake: None,
                effects: None,
            }
        }
    }

    pub fn set_snake(&mut self, snake: MaterialMesh2dBundle<ColorMaterial>) -> () {
        self.snake = Some(snake)
    }

    pub fn get_snake(&self) -> &MaterialMesh2dBundle<ColorMaterial> {
        self.snake.as_ref().expect("snake not set")
    }
}
