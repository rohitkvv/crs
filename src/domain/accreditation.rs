use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct Accreditation {
    pub id: Uuid,
    pub name: String,
    pub institution: String,
    pub start_date: DateTime<Utc>,
    pub end_date: Option<DateTime<Utc>>,
    pub status: AccreditationStatus,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum AccreditationStatus {
    Pending,
    Active,
    Expired,
    Revoked,
}

impl AccreditationStatus {
    /// Converts a string literal to an AccreditationStatus
    ///
    /// # Panics
    ///
    /// Panics if the status is not valid
    pub fn from_str(status: &str) -> AccreditationStatus {
        match status {
            "Pending" | "pending" => AccreditationStatus::Pending,
            "Active" | "active" => AccreditationStatus::Active,
            "Expired" | "expired" => AccreditationStatus::Expired,
            "Revoked" | "revoked" => AccreditationStatus::Revoked,
            _ => panic!("Invalid accreditation status"),
        }
    }
}

impl std::fmt::Display for AccreditationStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AccreditationStatus::Pending => write!(f, "Pending"),
            AccreditationStatus::Active => write!(f, "Active"),
            AccreditationStatus::Expired => write!(f, "Expired"),
            AccreditationStatus::Revoked => write!(f, "Revoked"),
        }
    }
}
