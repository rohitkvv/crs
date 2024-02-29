use actix_web::{body::BoxBody, http::header::ContentType, HttpRequest, HttpResponse, Responder};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{db::CertificateModel, dto::certificate_dto::CertificateDto};

use super::certificate_metadata::Metadata;

#[derive(Serialize, Deserialize, Debug)]
pub struct Certificate {
    pub id: Uuid,
    pub user_id: Uuid,
    pub account_id: u32,
    pub product_id: u32,
    pub metadata: Metadata,
    pub created_date: DateTime<Utc>,
    pub updated_date: Option<DateTime<Utc>>,
}

/// Implement the responder for the Certificate
impl Responder for Certificate {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body_result = serde_json::to_string(&self);

        // Create response and set content type
        match body_result {
            Ok(body) => HttpResponse::Ok()
                .content_type(ContentType::json())
                .body(body),
            Err(_) => HttpResponse::BadRequest().body("Unable to serialize the response"),
        }
    }
}

impl Certificate {
    /// Converts the certificate dto to a certificate
    /// # Examples
    ///
    /// ```
    /// use pretty_assertions::assert_eq;
    /// use uuid::Uuid;
    /// use crs::domain::certificate::Certificate;
    /// use crs::{dto::certificate_dto::CertificateDto, dto::certificate_metadata_dto::CertificateMetadataDto};
    ///
    /// let certificate_dto = CertificateDto {
    ///     user_id: Uuid::new_v4(),
    ///     account_id: 1,
    ///     product_id: 1,
    ///     metadata: CertificateMetadataDto {
    ///         score: 0,
    ///         progress: 0.5,
    ///         acquired_date: None,
    ///     },
    /// };
    /// let user_id = certificate_dto.user_id;
    /// let certificate = Certificate::from_dto(certificate_dto);
    ///
    /// assert_eq!(certificate.user_id, user_id);
    /// assert!(certificate.created_date.timestamp() > 0);
    /// ```
    pub fn from_dto(certificate: CertificateDto) -> Certificate {
        Certificate {
            id: Uuid::new_v4(),
            user_id: certificate.user_id,
            account_id: certificate.account_id,
            product_id: certificate.product_id,
            metadata: Metadata {
                score: certificate.metadata.score,
                progress: certificate.metadata.progress,
                acquired_date: certificate.metadata.acquired_date,
            },
            created_date: Utc::now(),
            updated_date: None,
        }
    }

    /// Converts the certificate model to a certificate
    /// # Examples
    ///
    /// ```
    /// use pretty_assertions::assert_eq;
    /// use mongodb::bson::{DateTime, Uuid};
    /// use crs::domain::certificate::Certificate;
    /// use crs::db::{CertificateModel, CertificateMetadataModel};
    /// use chrono::Utc;
    ///
    /// let certificate_model = CertificateModel {
    ///     certificate_id: Uuid::default(),
    ///     user_id: Uuid::default(),
    ///     account_id: 1,
    ///     product_id: 1,
    ///     metadata: CertificateMetadataModel {
    ///         score: 0,
    ///         progress: 0.5,
    ///         acquired_date: None,
    ///     },
    ///     created_date: DateTime::from_chrono(Utc::now()),
    ///     updated_date: None,
    /// };
    /// let certificate_id = certificate_model.certificate_id;
    /// let certificate = Certificate::from_model(certificate_model);
    /// assert_eq!(Uuid::from_uuid_1(certificate.id), certificate_id);
    /// ```
    pub fn from_model(certificate: CertificateModel) -> Certificate {
        Certificate {
            id: certificate.certificate_id.into(),
            user_id: certificate.user_id.into(),
            account_id: certificate.account_id,
            product_id: certificate.product_id,
            metadata: Metadata {
                score: certificate.metadata.score,
                progress: certificate.metadata.progress,
                acquired_date: certificate.metadata.acquired_date.map(|dt| dt.into()),
            },
            created_date: certificate.created_date.into(),
            updated_date: certificate.updated_date.map(|dt| dt.into()),
        }
    }
}
