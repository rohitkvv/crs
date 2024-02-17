mod domain;
mod handlers;

use actix_web::{web, HttpResponse};
use handlers::{get_certificate, store_certificate};

pub fn crs_service(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/certificates")
        .route(web::get().to(HttpResponse::MethodNotAllowed))
        .route(web::post().to(store_certificate::index))
        .route(web::head().to(HttpResponse::MethodNotAllowed))
    )
    .service(
        web::resource("/certificates/{certificate_id}")
            .route(web::get().to(get_certificate::by_id))
            .route(web::head().to(HttpResponse::MethodNotAllowed)),
    );
}
