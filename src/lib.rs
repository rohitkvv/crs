mod domain;
mod handlers;

use actix_web::{web, HttpResponse};
use handlers::get_certificate;

pub fn crs_service(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/{certificate_id}")
            .route(web::get().to(get_certificate::by_id))
            .route(web::head().to(HttpResponse::MethodNotAllowed)),
    )
    .service(web::resource("").route(web::head().to(HttpResponse::MethodNotAllowed)));
}
