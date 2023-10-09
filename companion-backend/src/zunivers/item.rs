use serde::Serialize;

#[derive(Serialize)]
pub struct Detail {
    pub counting: bool,
    pub craftable: bool,
    pub invocable: bool,
    pub recyclable: bool,
    pub tradable: bool,
    pub upgradable: bool
}

#[derive(Serialize)]
pub struct EvolutionDetail {
    pub owned: bool,
    pub item: Item
}

#[derive(Serialize)]
pub struct Item {
    pub id: String,
    pub pack: Pack
}

#[derive(Serialize)]
pub struct Fusion {

}

#[derive(Serialize)]
pub struct Pack {
    pub name: String
}