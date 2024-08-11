use serde::Deserialize;

use super::{certificate_metadata_dto::CertificateMetadataDto, recipient_dto::RecipientDto};

/// Certificate data transfer object
#[derive(Deserialize)]
pub struct CertificateDto {
    pub account_id: u32,
    pub product_id: u32,
    pub recipient: RecipientDto,
    pub metadata: CertificateMetadataDto,
}

impl CertificateDto {
    /// Validates the certificate dto and indicates if it is valid or not
    ///
    /// # Returns
    /// **true** if the certificate dto is valid, otherwise **false**
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// use pretty_assertions::assert_eq;
    /// use crs::dto::{certificate_dto::CertificateDto, certificate_metadata_dto::CertificateMetadataDto, recipient_dto::RecipientDto};
    /// use uuid::Uuid;
    ///
    /// let certificate = CertificateDto {
    ///     account_id: 1,
    ///     product_id: 1,
    ///     recipient: RecipientDto {
    ///         id: Uuid::new_v4(),
    ///         first_name: "firstName".to_string(),
    ///         last_name: "lastName".to_string(),
    ///         email: "test@email.com".to_string(),
    ///         phone: "12345678".to_string()
    ///     },
    ///     metadata: CertificateMetadataDto {
    ///         score: 0,
    ///         progress: 0.5,
    ///         acquired_date: None,
    ///         accreditation: None,
    ///     },
    /// };
    ///
    /// assert_eq!(certificate.is_valid(), true);
    ///
    /// ```
    ///
    /// ---
    ///
    /// ```
    ///
    /// use pretty_assertions::assert_eq;
    /// use crs::dto::{certificate_dto::CertificateDto, certificate_metadata_dto::CertificateMetadataDto, recipient_dto::RecipientDto};
    /// use uuid::Uuid;
    ///
    /// let certificate = CertificateDto {
    ///     account_id: 1,
    ///     product_id: 1,
    ///     recipient: RecipientDto {
    ///         id: Uuid::nil(),
    ///         first_name: "firstName".to_string(),
    ///         last_name: "lastName".to_string(),
    ///         email: "test@email.com".to_string(),
    ///         phone: "12345678".to_string()
    ///     },
    ///     metadata: CertificateMetadataDto {
    ///         score: 0,
    ///         progress: 0.5,
    ///         acquired_date: None,
    ///         accreditation: None,
    ///     },
    /// };
    ///
    /// assert_eq!(certificate.is_valid(), false);
    ///
    /// ```
    pub fn is_valid(&self) -> bool {
        !self.account_id != 0
            && self.product_id != 0
            && self.recipient.is_valid()
            && self.metadata.is_valid()
    }
}
