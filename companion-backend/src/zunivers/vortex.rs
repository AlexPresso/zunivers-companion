use serde::Serialize;

#[derive(Serialize)]
pub struct Activity {
    pub stats: Vec<Stats>
}

#[derive(Serialize)]
pub struct Stats {
    pub tower_log_count: u16,
    pub floor_index: u8,
    pub begin_date: String, //TODO: Change me to LocalDate
    pub end_date: String //TODO: same
}

#[derive(Serialize)]
pub struct Detail {
    pub pack_id: String
}