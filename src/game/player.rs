use crate::game::effect::CurvEffect;

#[derive(Default)]
pub struct Player {
    x: usize,
    y: usize,
    effects: Box<dyn CurvEffect>,
}
