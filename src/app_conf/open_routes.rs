use actix_web::{web, Scope};
use crate::handlers::*;

pub fn get_all() -> Scope {
    web::scope("/api-open")
        .service(
            web::resource("/password")
                .route(web::post().to(auth::ask_password_reset))
                .route(web::put().to(auth::reset_password)))
}