use actix_web::{web, Scope};
use crate::handlers::*;

pub fn get_all() -> Scope {
    web::scope("/api")
        .service(
    web::resource("/auth")
                .route(web::post().to(auth::login))
    )
}