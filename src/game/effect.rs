#[derive(Default)]
pub enum Effect {
    #[default]
    Default,
    ThroughWalls,
}

pub trait CurvEffect: Sync + Send {
    fn apply(&self);
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
        println!("application du bonus");
    }
}

impl CurvEffect for Malus {
    fn apply(&self) {
        println!("Application du malus")
    }
}

impl Default for Box<dyn CurvEffect> {
    fn default() -> Self {
        return Box::new(Bonus { effect: Effect::Default });
    }
}