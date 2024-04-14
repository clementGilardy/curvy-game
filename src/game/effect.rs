pub enum Effect {
    ThroughWalls
}

pub trait CurvEffect: Sync + Send {
    fn apply(&self);
}

struct Bonus {
    effect: Effect,
}

struct Malus {
    effect: Effect,
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