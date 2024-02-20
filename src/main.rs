use actix_web::{middleware, web, App, HttpServer};
use crs::{crs_service, db};
use dotenvy::dotenv;

use log::info;
use db::init_db;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    info!("Initializing CRS!");

    let db = init_db().await;

    HttpServer::new(move || {
        App::new()
            // enable logger
            .wrap(middleware::Logger::default())
            .app_data(web::Data::new(db.clone()))
            // configure services
            .configure(crs_service)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
