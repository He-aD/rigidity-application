use actix_web::{web, Scope};
use crate::handlers::{auth, user};

pub fn get_all() -> Scope {
    web::scope("/api-open")
        .service(
            web::resource("/password")
                .route(web::post().to(auth::ask_password_reset))
                .route(web::put().to(auth::reset_password)))
        .service(
            web::resource("/login")
                .route(web::post().to(auth::login)))
        .service(
            web::resource("/login-steam")
                .route(web::post().to(auth::login_steam)))
        .service(
            web::resource("/user/create")
                .route(web::post().to(user::create)))
}