use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Validity {
    pub first_valid_from: DateTime<Utc>,
    pub valid_from: DateTime<Utc>,
    pub valid_until: ValidUntil,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ValidUntil {
    EndOfTime,
    Expiry(DateTime<Utc>),
}
