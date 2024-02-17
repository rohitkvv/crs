use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Serialize)]
pub struct Metadata {
    pub score: u32,
    pub progress: u32,
    pub pe_points: u32,
    pub acquired_date: DateTime<Utc>,
}