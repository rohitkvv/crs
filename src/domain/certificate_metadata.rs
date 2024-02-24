use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Metadata {
    pub score: u32,
    pub progress: u32,
    pub pe_points: u32,
    pub acquired_date: Option<DateTime<Utc>>,
}
