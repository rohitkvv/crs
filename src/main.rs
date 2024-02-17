use actix_web::{web, App, HttpServer, middleware};
use dotenvy::dotenv;
use log::info;
use crs::crs_service;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    info!("Initializing CRS!");

    HttpServer::new(|| {
        App::new()
            // enable logger
            .wrap(middleware::Logger::default())
            // register scoped services
            .service(web::scope("/api/certificates").configure(crs_service))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
