use actix_web::{web, Scope};
use crate::handlers::auth;

pub fn get_all() -> Scope {
    web::scope("/api-open")
        .service(
            web::resource("/password")
                .route(web::post().to(auth::ask_password_reset))
                .route(web::put().to(auth::reset_password)))
        .service(
            web::resource("/login")
                .route(web::post().to(auth::login)))
}