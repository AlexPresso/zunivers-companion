use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Event {
    pub balance_cost: u16,
    pub name: String,
    pub begin_date: String,
    pub end_date: String,
    pub one_time: bool,
    pub pack_id: String
}