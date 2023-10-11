use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Challenge {
    pub id: String,
    pub challenge_type: Type,
    pub reward_lore_dust: u16,
    pub progress: Progress,
    pub challenge_log: Log,
    pub description: String
}

#[derive(Serialize, Deserialize)]
pub struct Progress {
    pub progress_type: String,
    pub current: u16,
    pub max: u16
}

#[derive(Serialize, Deserialize)]
pub struct Log {
    pub id: String
}

#[derive(Serialize, Deserialize)]
pub enum Type {
    FUSE,
    INVOKE,
    ASCEND,
    TRADE,
    UPGRADE,
    CRAFT,
    META_CHALLENGE,
    LOOT,
    RECYCLE,
    INVOKE_ITEMS,
    TRADE_USERS,
    LUCKY,
    WEEKLY
}