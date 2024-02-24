use actix_web::{web, Either, HttpResponse, Responder};
use log::{error, info};
use mongodb::Database;
use uuid::Uuid;

use crate::{
    db::find_certificate_by_id,
    domain::certificate::Certificate,
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
    use actix_web::{
        dev::Service,
        http::StatusCode,
        test::{self},
        web, App,
    };
    use chrono::Utc;
    use uuid::Uuid;

    use crate::{
        crs_service,
        db::{init_db, CertificateModel}, domain::{certificate::Certificate, certificate_metadata::Metadata},
    };

    #[actix_web::test]
    #[ignore = "requires MongoDB instance running"]
    async fn find_certificate_by_valid_id() {
        let db = init_db().await.expect("failed to connect");
        // Clear any data currently in the certificates collection.
        let coll = &db.collection::<CertificateModel>("certificates");
        coll
            .drop(None)
            .await
            .expect("drop collection should succeed");

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(Some(db)))
                .configure(crs_service),
        )
        .await;

        let certificate = Certificate {
            id: Uuid::new_v4(),
            user_id: Uuid::new_v4(),
            account_id: Uuid::new_v4(),
            product_id: 1,
            metadata: Metadata {
                score: 50,
                progress: 50,
                pe_points: 1,
                acquired_date: Some(Utc::now()),
            },
            created_date: Utc::now(),
            updated_date: None, 
        };

        let certificate_model = CertificateModel::convert(&certificate);
        // Insert a document into the collection.
        coll
            .insert_one(certificate_model, None)
            .await
            .expect("insert one into collection should succeed");
        
        let certificate_id = certificate.id;
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
