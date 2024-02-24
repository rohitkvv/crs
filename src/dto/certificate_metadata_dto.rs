use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CertificateMetadataDto {
    pub score: u32,
    pub progress: u32,
    pub acquired_date: Option<DateTime<Utc>>,
}
