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
    pub account_id: Uuid,
    pub product_id: u32,
    pub metadata: Metadata,
    pub created_date: DateTime<Utc>,
    pub updated_date: Option<DateTime<Utc>>,
}

// Responder
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
    pub fn from_dto(certificate: CertificateDto) -> Certificate {
        Certificate {
            id: Uuid::new_v4(),
            user_id: certificate.user_id,
            account_id: certificate.account_id,
            product_id: certificate.product_id,
            metadata: Metadata {
                score: certificate.metadata.score,
                progress: certificate.metadata.progress,
                pe_points: 0,
                acquired_date: match certificate.metadata.acquired_date {
                    Some(dt) => Some(dt),
                    None => None,
                },
            },
            created_date: Utc::now(),
            updated_date: None,
        }
    }

    pub fn from_model(certificate: CertificateModel) -> Certificate {
        Certificate {
            id: certificate.certificate_id.into(),
            user_id: certificate.user_id.into(),
            account_id: certificate.account_id.into(),
            product_id: certificate.product_id,
            metadata: Metadata {
                score: certificate.metadata.score,
                progress: certificate.metadata.progress,
                pe_points: 0,
                acquired_date: match certificate.metadata.acquired_date {
                    Some(dt) => Some(dt.into()),
                    None => None,
                },
            },
            created_date: certificate.created_date.into(),
            updated_date: match certificate.updated_date {
                Some(dt) => Some(dt.into()),
                None => None,
            },
        }
    }
}
