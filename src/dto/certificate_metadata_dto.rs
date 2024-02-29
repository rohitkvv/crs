use chrono::{DateTime, Utc};
use serde::Deserialize;

/// Certificate metadata data transfer object
#[derive(Deserialize)]
pub struct CertificateMetadataDto {
    pub score: u32,
    pub progress: f32,
    pub acquired_date: Option<DateTime<Utc>>,
}

impl CertificateMetadataDto {
    /// Validates the certificate metadata
    /// # Examples
    ///
    /// ```
    /// use pretty_assertions::assert_eq;
    /// use crs::dto::certificate_metadata_dto::CertificateMetadataDto;
    ///
    /// let metadata = CertificateMetadataDto {
    ///     score: 0,
    ///     progress: 0.5,
    ///     acquired_date: None,
    /// };
    /// assert_eq!(metadata.is_valid(), true);
    /// ```
    ///
    /// ```
    /// use pretty_assertions::assert_eq;
    /// use crs::dto::certificate_metadata_dto::CertificateMetadataDto;
    ///
    /// let metadata = CertificateMetadataDto {
    ///     score: 0,
    ///     progress: 1.5,
    ///     acquired_date: None,
    /// };
    /// assert_eq!(metadata.is_valid(), false);
    /// ```
    pub fn is_valid(&self) -> bool {
        self.score <= 100 && self.progress >= 0.0 && self.progress <= 1.0
    }
}
