use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Detail {
    pub counting: bool,
    pub craftable: bool,
    pub invocable: bool,
    pub recyclable: bool,
    pub tradable: bool,
    pub upgradable: bool
}

#[derive(Serialize, Deserialize)]
pub struct EvolutionDetail {
    pub owned: bool,
    pub item: Item
}

#[derive(Serialize, Deserialize)]
pub struct Item {
    pub id: String,
    pub pack: Pack
}

#[derive(Serialize, Deserialize)]
pub struct Fusion {

}

#[derive(Serialize, Deserialize)]
pub struct Pack {
    pub name: String
}