use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::accreditation::Accreditation;

#[derive(Serialize, Deserialize, Debug)]
pub struct Metadata {
    pub score: u32,
    pub progress: f32,
    pub acquired_date: Option<DateTime<Utc>>,
    pub accreditation: Option<Accreditation>,
}
