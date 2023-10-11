use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    pub discord_id: String,
    pub discord_usern_name: String,
    pub position: u32,
    pub lore_dust: u32,
    pub lore_fragment: u32,
    pub upgrade_dust: u32,
    pub balance: u32,
    pub statistics: Statistics
}

#[derive(Serialize, Deserialize)]
pub struct Statistics {
    pub achievement_count: u16,
    pub achievement_log_count: u16,
    pub inventory_count: u32,
    pub inventory_unique_count: u16,
    pub inventory_unique_golden_count: u16,
    pub item_count: u32,
    pub lucky_count: u32,
    pub trade_count: u32,
    pub subscibed: bool
}