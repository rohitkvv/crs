use actix_web::{web, Either, HttpResponse, Responder};
use chrono::{DateTime, Utc};
use log::{error, info};
use mongodb::Database;
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    db::{store_one, CertificateModel},
    domain::{certificate::Certificate, certificate_metadata::Metadata},
};

#[derive(Deserialize)]
pub(crate) struct CertificateDto {
    pub(crate) user_id: Uuid,
    pub(crate) account_id: Uuid,
    pub(crate) product_id: u32,
    pub(crate) metadata: CertMetadataDto,
}

#[derive(Deserialize)]
pub(crate) struct CertMetadataDto {
    pub(crate) score: u32,
    pub(crate) progress: u32,
    pub(crate) acquired_date: DateTime<Utc>,
}

pub async fn index(
    certificate: web::Json<CertificateDto>,
    data: web::Data<Option<Database>>,
) -> impl Responder {
    // Implement proper validation
    if Uuid::is_nil(&certificate.user_id) || Uuid::is_nil(&certificate.account_id) {
        Either::Right(HttpResponse::BadRequest().body("Invalid ceritificate"))
    } else {
        match data.into() {
            Some(db) => {
                let sample_cert = Certificate {
                    id: Uuid::new_v4(),
                    user_id: certificate.user_id,
                    account_id: certificate.account_id,
                    product_id: certificate.product_id,
                    metadata: Metadata {
                        score: certificate.metadata.score,
                        progress: certificate.metadata.progress,
                        pe_points: 0,
                        acquired_date: certificate.metadata.acquired_date,
                    },
                    created_date: Utc::now(),
                    updated_date: Utc::now(),
                };
                let doc = CertificateModel::convert(&sample_cert);
                if let Some(database) = db.as_ref() {
                    if let Some(insert_one_result) = store_one(database, &doc).await {
                        info!(
                            "The inserted record id is: {}",
                            insert_one_result.inserted_id
                        );
                        return Either::Left(sample_cert);
                    }
                }
                return Either::Right(HttpResponse::InternalServerError().body("Failed to store certificate!"));
            }
            None => {
                error!("Invalid db instance");
                return Either::Right(HttpResponse::BadRequest().body("Invalid ceritificate"));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use actix_web::{
        dev::Service, http::{header, StatusCode}, test, web, App
    };
    

    use crate::{crs_service, db::{init_db, CertificateModel}};

    #[actix_web::test]
    #[ignore = "requires MongoDB instance running"]
    async fn post_valid_certificate() {
        let db = init_db().await.expect("failed to connect");

        // Clear any data currently in the users collection.
        db
            .collection::<CertificateModel>("certificates")
            .drop(None)
            .await
            .expect("drop collection should succeed");

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(Some(db)))
                .configure(crs_service)
            )
            .await;

        let payload = r#"{"user_id":"a2382a52-2e84-4db6-bcd9-4fe378a92b10","account_id":"7b80ffe8-910d-44fb-9a68-ff7c0eaba767","product_id":1,"metadata":{"score":100,"progress":100,"acquired_date":"2023-11-28T12:45:59.324310806Z"}}"#.as_bytes();

        let req = test::TestRequest::post()
            .insert_header((header::CONTENT_TYPE, "application/json"))
            .uri("/api/certificates")
            .set_payload(payload)
            .to_request();

        let resp = app.call(req).await.unwrap();

        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_web::test]
    #[ignore = "requires MongoDB instance running"]
    async fn post_invalid_certificate_should_return_bad_request() {
        let db = init_db().await.expect("failed to connect");

        // Clear any data currently in the users collection.
        db
            .collection::<CertificateModel>("certificates")
            .drop(None)
            .await
            .expect("drop collection should succeed");

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(Some(db)))
                .configure(crs_service)
            )
            .await;
        let payload = r#"{"user_id":"00000000-0000-0000-0000-000000000000","account_id":"7b80ffe8-910d-44fb-9a68-ff7c0eaba767","product_id":1,"metadata":{"score":100,"progress":100,"acquired_date":"2023-11-28T12:45:59.324310806Z"}}"#.as_bytes();

        let req = test::TestRequest::post()
            .insert_header((header::CONTENT_TYPE, "application/json"))
            .uri("/api/certificates")
            .set_payload(payload)
            .to_request();

        let resp = app.call(req).await.unwrap();

        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }
}
