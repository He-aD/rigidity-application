use actix_web::{web, Scope};
use crate::auth_handler;

pub fn get_all() -> Scope {
    web::scope("/api")
    //     .service(
    // web::resource("/ask-password-reset")
    //             .route(web::post().to(auth_handler::ask_password_reset))
    //     )
    //     .service(
    // web::resource("/auth")
    //             .route(web::post().to(auth_handler::login))
    //             .route(web::delete().to(auth_handler::logout))
    //             .route(web::get().to(auth_handler::get_me)),
    //     )
}