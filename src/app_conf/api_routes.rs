use actix_web::{web, Scope};
use crate::handlers::auth;

pub fn get_all() -> Scope {
    web::scope("/api").service(
        web::resource("/logout")
            .route(web::post().to(auth::logout)))
}