use chrono::{DateTime, Utc};
use serde::Deserialize;

/// Certificate metadata data transfer object
#[derive(Deserialize)]
pub struct CertificateMetadataDto {
    pub score: u32,
    pub progress: f32,
    pub acquired_date: Option<DateTime<Utc>>,
    pub accreditation: Option<AccreditationDto>,
}

impl CertificateMetadataDto {
    /// Validates the certificate metadata
    /// # Returns
    /// **true** if the certificate metadata is valid, otherwise **false**
    ///
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
    ///     accreditation: None,
    /// };
    /// assert_eq!(metadata.is_valid(), true);
    /// ```
    /// ---
    ///
    /// ```
    ///
    /// use pretty_assertions::assert_eq;
    /// use crs::dto::certificate_metadata_dto::CertificateMetadataDto;
    ///
    /// let metadata = CertificateMetadataDto {
    ///     score: 0,
    ///     progress: 1.5,
    ///     acquired_date: None,
    ///     accreditation: None,
    /// };
    /// assert_eq!(metadata.is_valid(), false);
    /// ```
    pub fn is_valid(&self) -> bool {
        self.score <= 100
            && self.progress >= 0.0
            && self.progress <= 1.0
            && AccreditationDto::is_valid_option(&self.accreditation)
    }
}

#[derive(Deserialize, Debug)]
pub struct AccreditationDto {
    pub name: String,
    pub institution: String,
    pub start_date: DateTime<Utc>,
    pub end_date: Option<DateTime<Utc>>,
    pub status: String,
}

impl AccreditationDto {
    /// Validates the option type of accreditation dto
    pub fn is_valid_option(acc_dto: &Option<AccreditationDto>) -> bool {
        match acc_dto {
            Some(acc) => {
                !acc.name.is_empty() && !acc.institution.is_empty() && !acc.status.is_empty()
            }
            None => true,
        }
    }
}
