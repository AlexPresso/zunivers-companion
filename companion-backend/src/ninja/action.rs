use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Action {
    pub action_type: Type,
    pub target: Element
}

pub enum Type {
    RECYCLE,
    CRAFT,
    ECHANT,
    FUSION,
    INVOCATION,
    ASCENSION,
    LUCKY_RAYOU,
    DAILY,
    WEEKLY,
    CONSTELLATION,
    EVOLUTION
}

pub trait Element {
    fn get_identifier(&self) -> String;
}