use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Activity {
    pub stats: Vec<Stats>
}

#[derive(Serialize, Deserialize)]
pub struct Stats {
    pub tower_log_count: u16,
    pub floor_index: u8,
    pub begin_date: String, //TODO: Change me to LocalDate
    pub end_date: String //TODO: same
}

#[derive(Serialize, Deserialize)]
pub struct Detail {
    pub pack_id: String
}