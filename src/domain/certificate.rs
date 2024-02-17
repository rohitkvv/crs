use actix_web::{body::BoxBody, http::header::ContentType, HttpRequest, HttpResponse, Responder};
use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

use super::certificate_metadata::Metadata;

#[derive(Serialize)]
pub struct Certificate {
    pub id: Uuid,
    pub user_id: Uuid,
    pub account_id: Uuid,
    pub product_id: u32,
    pub metadata: Metadata,
    pub created_date: DateTime<Utc>,
    pub updated_date: DateTime<Utc>,
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
