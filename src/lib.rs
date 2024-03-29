pub mod db;
pub mod domain;
pub mod dto;
mod handlers;
mod helpers;
pub mod model;

use actix_web::{web, HttpResponse};
use handlers::{get_certificate, store_certificate};

pub fn crs_service(cfg: &mut web::ServiceConfig) {
    // register scoped services
    cfg.service(
        web::scope("/api/certificates")
            .service(
                web::resource("")
                    .route(web::post().to(store_certificate::index))
                    .route(web::head().to(HttpResponse::MethodNotAllowed)),
            )
            .service(
                web::resource("/{certificate_id}")
                    .route(web::get().to(get_certificate::by_id))
                    .route(web::head().to(HttpResponse::MethodNotAllowed)),
            )
            .service(
                web::resource("/user/{user_id}")
                    .route(web::get().to(get_certificate::by_user_id))
                    .route(web::head().to(HttpResponse::MethodNotAllowed)),
            ),
    );
}
