use actix_web::{web, Either, HttpResponse, Responder};
use chrono::Utc;
use uuid::Uuid;

use crate::domain::{certificate::Certificate, certificate_metadata::Metadata};

pub async fn by_id(path: web::Path<(Uuid,)>) -> impl Responder {
    let certificate_id = path.into_inner().0;

    // Implement proper validation
    if Uuid::is_nil(&certificate_id) {
        Either::Right(HttpResponse::BadRequest().body("Invalid ceritificate id"))
    } else {
        Either::Left(Certificate {
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
        })
    }
}

#[cfg(test)]
mod tests {
    use actix_web::{dev::Service, http::StatusCode, test, App};
    use uuid::Uuid;

    use crate::crs_service;

    #[actix_web::test]
    async fn test_get_certificate_by_id() {
        let app = test::init_service(App::new().configure(crs_service)).await;

        let certificate_id = Uuid::new_v4();

        let req = test::TestRequest::get()
            .uri(&format!("/{certificate_id}"))
            .to_request();

        let resp = app.call(req).await.unwrap();

        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_get_certificate_by_invalid_id() {
        let app = test::init_service(App::new().configure(crs_service)).await;

        let certificate_id = Uuid::nil();

        let req = test::TestRequest::get()
            .uri(&format!("/{certificate_id}"))
            .to_request();

        let resp = app.call(req).await.unwrap();

        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }
}
