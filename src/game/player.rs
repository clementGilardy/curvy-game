use crate::game::effect::CurvEffect;

#[derive(Default, Debug)]
pub struct Player {
    id: u8,
    pub x: usize,
    pub y: usize,
    effects: Vec<Box<dyn CurvEffect>>,
}

impl Player {
    pub fn new() -> Self {
        static mut NEXT_ID: u8 = 0;
        unsafe {
            NEXT_ID += 1;
            Player {
                id: NEXT_ID,
                x: 0,
                y: 0,
                effects: Vec::new(),
            }
        }
    }
}
