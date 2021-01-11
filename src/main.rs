
use actix_web::{App, HttpServer};
use std::{fs, io};

mod app_conf;
mod auth_handler;
mod errors;
mod models;
mod schema;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    app_conf::set_env();

    HttpServer::new(move || {
        App::new()
            .data(app_conf::connect_database())
            // enable logger
            .wrap(app_conf::middleware_logger())
            .wrap(app_conf::middleware_identity_service())
            // everything under '/api/' route
            .service(app_conf::api_routes::get_all())
            .service(app_conf::static_routes::get_all())
            .default_service(app_conf::static_routes::default_service()) // 404
    })
    .bind("127.0.0.1:3000")?
    .run()
    .await
}