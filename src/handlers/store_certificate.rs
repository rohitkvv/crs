use actix_web::{web, Either, HttpResponse, Responder};
use log::{error, info};
use mongodb::Database;
use uuid::Uuid;

use crate::{
    db::{store_one, CertificateModel},
    domain::certificate::Certificate,
    dto::certificate_dto::CertificateDto,
    helpers::SaveType,
};

pub async fn index(
    certificate: web::Json<CertificateDto>,
    data: web::Data<Option<Database>>,
) -> impl Responder {
    if !CertificateDto::is_valid(&certificate)
    {
        Either::Right(HttpResponse::BadRequest().body("Invalid ceritificate"))
    } else {
        match data.into() {
            Some(db) => {
                let cert_to_store = Certificate::from_dto(certificate.0);
                let doc = CertificateModel::convert(&cert_to_store, SaveType::Insert);
                if let Some(database) = db.as_ref() {
                    if let Some(insert_one_result) = store_one(database, &doc).await {
                        info!(
                            "The inserted record id is: {}",
                            insert_one_result.inserted_id
                        );
                        return Either::Left(cert_to_store);
                    }
                }
                Either::Right(
                    HttpResponse::InternalServerError().body("Failed to store certificate!"),
                )
            }
            None => {
                error!("Invalid db instance");
                Either::Right(HttpResponse::BadRequest().body("Invalid ceritificate"))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use actix_web::{
        dev::Service,
        http::{header, StatusCode},
        test, web, App,
    };

    use crate::{
        crs_service,
        db::{init_db, CertificateModel},
    };

    #[actix_web::test]
    #[ignore = "requires MongoDB instance running"]
    async fn post_valid_certificate() {
        let db = init_db().await.expect("failed to connect");

        // Clear any data currently in the users collection.
        db.collection::<CertificateModel>("certificates")
            .drop(None)
            .await
            .expect("drop collection should succeed");

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(Some(db)))
                .configure(crs_service),
        )
        .await;

        let payload = r#"{"user_id":"a2382a52-2e84-4db6-bcd9-4fe378a92b10","account_id":20,"product_id":15,"metadata":{"score":100,"progress":100,"acquired_date":"2023-11-28T12:45:59.324310806Z"}}"#.as_bytes();

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
        db.collection::<CertificateModel>("certificates")
            .drop(None)
            .await
            .expect("drop collection should succeed");

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(Some(db)))
                .configure(crs_service),
        )
        .await;
        let payload = r#"{"user_id":"00000000-0000-0000-0000-000000000000","account_id":20,"product_id":15,"metadata":{"score":100,"progress":100,"acquired_date":"2023-11-28T12:45:59.324310806Z"}}"#.as_bytes();

        let req = test::TestRequest::post()
            .insert_header((header::CONTENT_TYPE, "application/json"))
            .uri("/api/certificates")
            .set_payload(payload)
            .to_request();

        let resp = app.call(req).await.unwrap();

        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }
}
