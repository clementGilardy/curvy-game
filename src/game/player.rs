use crate::game::effect::CurvEffect;

pub struct Player {
    x: usize,
    y: usize,
    effects: Box<dyn CurvEffect>,
}
