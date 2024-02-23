use actix_web::{web, Either, HttpResponse, Responder};
use chrono::Utc;
use log::{error, info};
use mongodb::Database;
use uuid::Uuid;

use crate::{
    db::find_certificate_by_id,
    domain::{certificate::Certificate, certificate_metadata::Metadata},
};

pub async fn by_id(path: web::Path<(Uuid,)>, data: web::Data<Option<Database>>) -> impl Responder {
    let certificate_id = path.into_inner().0;

    // Implement proper validation
    if Uuid::is_nil(&certificate_id) {
        Either::Right(HttpResponse::BadRequest().body("Invalid ceritificate id"))
    } else {
        match data.into() {
            Some(db) => {
                if let Some(database) = db.as_ref() {
                    if let Some(find_result) =
                        find_certificate_by_id(database, certificate_id).await
                    {
                        info!("{:?}", find_result);
                        return Either::Left(Certificate {
                            id: Uuid::new_v4(),
                            user_id: Uuid::new_v4(),
                            account_id: Uuid::new_v4(),
                            product_id: 1,
                            metadata: Metadata {
                                score: 100,
                                progress: 100,
                                pe_points: 0,
                                acquired_date: Utc::now(),
                            },
                            created_date: Utc::now(),
                            updated_date: Utc::now(),
                        });
                    }
                }

                Either::Right(
                    HttpResponse::InternalServerError().body("Failed to find certificate!"),
                )
            }
            None => {
                error!("Unable to read state data");
                Either::Right(HttpResponse::InternalServerError().body("DB State is unavailable"))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use actix_web::{
        dev::Service,
        http::{header, StatusCode},
        test::{self, call_and_read_body_json},
        web, App,
    };
    use uuid::Uuid;

    use crate::{
        crs_service,
        db::{init_db, CertificateModel},
    };

    #[actix_web::test]
    #[ignore = "requires MongoDB instance running"]
    async fn find_certificate_by_valid_id() {
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

        let payload = r#"{"user_id":"a2382a52-2e84-4db6-bcd9-4fe378a92b10","account_id":"7b80ffe8-910d-44fb-9a68-ff7c0eaba767","product_id":1,"metadata":{"score":100,"progress":100,"acquired_date":"2023-11-28T12:45:59.324310806Z"}}"#.as_bytes();

        let post_req = test::TestRequest::post()
            .insert_header((header::CONTENT_TYPE, "application/json"))
            .uri("/api/certificates")
            .set_payload(payload)
            .to_request();

        let post_response: CertificateModel = call_and_read_body_json(&app, post_req).await;

        let certificate_id = post_response.certificate_id;

        let req = test::TestRequest::get()
            .uri(&format!("/api/certificates/{certificate_id}"))
            .to_request();

        let resp = app.call(req).await.unwrap();

        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_web::test]
    #[ignore = "requires MongoDB instance running"]
    async fn test_get_certificate_by_invalid_id() {
        let db = init_db().await.expect("failed to connect");
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(Some(db)))
                .configure(crs_service),
        )
        .await;

        let certificate_id = Uuid::nil();

        let req = test::TestRequest::get()
            .uri(&format!("/api/certificates/{certificate_id}"))
            .to_request();

        let resp = app.call(req).await.unwrap();

        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }
}
