use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::error::AccreditationStatusError;

#[derive(Serialize, Deserialize, Debug)]
pub struct Accreditation {
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
    pub fn from_status_str(status: &str) -> Result<AccreditationStatus, AccreditationStatusError> {
        match status {
            "Pending" | "pending" => Ok(AccreditationStatus::Pending),
            "Active" | "active" => Ok(AccreditationStatus::Active),
            "Expired" | "expired" => Ok(AccreditationStatus::Expired),
            "Revoked" | "revoked" => Ok(AccreditationStatus::Revoked),
            _ => Err(AccreditationStatusError),
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
