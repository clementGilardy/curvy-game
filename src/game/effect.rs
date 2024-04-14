use std::fmt::{Debug, Formatter};

#[derive(Default, Debug)]
pub enum Effect {
    #[default]
    Default,
    ThroughWalls,
}

pub trait CurvEffect: Sync + Send {
    fn apply(&self);
    fn get_effect(&self) -> &Effect;
}

pub struct Bonus {
    effect: Effect,
}

pub struct Malus {
    effect: Effect,
}

impl Bonus {
    pub fn new() -> Self {
        Bonus {
            effect: Effect::Default
        }
    }
}

impl CurvEffect for Bonus {
    fn apply(&self) {
        println!("application du bonus {:?}", self.effect);
    }

    fn get_effect(&self) -> &Effect {
        return &self.effect;
    }
}

impl CurvEffect for Malus {

    fn apply(&self) {
        println!("Application du malus {:?}", self.effect)
    }

    fn get_effect(&self) -> &Effect {
        return &self.effect;
    }
}

impl Default for Box<dyn CurvEffect> {
    fn default() -> Self {
        return Box::new(Bonus { effect: Effect::Default });
    }
}

impl Debug for Box<dyn CurvEffect> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "rien")
    }
}