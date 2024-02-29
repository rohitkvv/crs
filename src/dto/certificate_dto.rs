use serde::Deserialize;
use uuid::Uuid;

use super::certificate_metadata_dto::CertificateMetadataDto;

/// Certificate data transfer object
#[derive(Deserialize)]
pub struct CertificateDto {
    pub user_id: Uuid,
    pub account_id: u32,
    pub product_id: u32,
    pub metadata: CertificateMetadataDto,
}

impl CertificateDto {
    /// Validates the certificate
    /// # Examples
    ///
    /// ```
    /// use pretty_assertions::assert_eq;
    /// use crs::dto::{certificate_dto::CertificateDto, certificate_metadata_dto::CertificateMetadataDto};
    /// use uuid::Uuid;
    ///
    /// let certificate = CertificateDto {
    ///     user_id: Uuid::new_v4(),
    ///     account_id: 1,
    ///     product_id: 1,
    ///     metadata: CertificateMetadataDto {
    ///         score: 0,
    ///         progress: 0.5,
    ///         acquired_date: None,
    ///     },
    /// };
    /// assert_eq!(certificate.is_valid(), true);
    /// ```
    ///
    /// ```
    /// use pretty_assertions::assert_eq;
    /// use crs::dto::{certificate_dto::CertificateDto, certificate_metadata_dto::CertificateMetadataDto};
    /// use uuid::Uuid;
    ///
    /// let certificate = CertificateDto {
    ///     user_id: Uuid::nil(),
    ///     account_id: 1,
    ///     product_id: 1,
    ///     metadata: CertificateMetadataDto {
    ///         score: 0,
    ///         progress: 0.5,
    ///         acquired_date: None,
    ///     },
    /// };
    /// assert_eq!(certificate.is_valid(), false);
    /// ```
    pub fn is_valid(&self) -> bool {
        !self.user_id.is_nil()
            && !self.user_id.is_max()
            && self.account_id != 0
            && self.product_id != 0
    }
}
