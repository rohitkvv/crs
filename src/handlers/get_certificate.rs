use actix_web::{web, Either, HttpResponse, Responder};
use log::{error, info};
use mongodb::Database;
use uuid::Uuid;

use crate::{db::find_certificate_by_id, domain::certificate::Certificate};

pub async fn by_id(path: web::Path<(Uuid,)>, data: web::Data<Option<Database>>) -> impl Responder {
    let certificate_id = path.into_inner().0;

    // Implement proper validation
    if Uuid::is_nil(&certificate_id) {
        Either::Right(HttpResponse::BadRequest().body("Invalid ceritificate id"))
    } else {
        match data.into() {
            Some(db) => {
                if let Some(database) = db.as_ref() {
                    if let Some(certificate_model) =
                        find_certificate_by_id(database, certificate_id).await
                    {
                        info!("{:?}", certificate_model);
                        return Either::Left(Certificate::from_model(certificate_model));
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
    use std::str::FromStr;

    use actix_web::{
        dev::Service,
        http::StatusCode,
        test::{self},
        web, App,
    };
    use uuid::Uuid;

    use crate::{
        crs_service,
        db::init_db,
    };

    #[actix_web::test]
    #[ignore = "requires MongoDB instance running"]
    async fn find_certificate_by_valid_id() {
        let db = init_db().await.expect("failed to connect");

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(Some(db)))
                .configure(crs_service),
        )
        .await;

        // Tried to insert a certificate into the database first and use the same id here,
        // but it didn't work. So, I'm using a random id here.
        // So, make sure to change the id to a valid one before running the test.
        let certificate_id = Uuid::from_str("4e7fc873-cdec-4701-9511-7b56ddede695").unwrap();
        let uri = format!("/api/certificates/{}", certificate_id);
        let req = test::TestRequest::with_uri(uri.as_str()).to_request();

        let resp = test::call_service(&app, req).await;
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
