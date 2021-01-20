use actix_web::{web, Scope};
use crate::handlers::{custom_room, auth};

pub fn get_all() -> Scope {
    web::scope("/api")
        .service(
            web::resource("/logout")
                .route(web::post().to(auth::logout)))
        .service(
            web::resource("/matchmaking/custom-room")
                .route(web::get().to(custom_room::get_all))
                .route(web::post().to(custom_room::create)))
        .service(
            web::resource("/matchmaking/custom-room/{id}/join")
                .route(web::put().to(custom_room::join)))
        .service(
            web::resource("/matchmaking/custom-room/{id}/quit")
                .route(web::put().to(custom_room::quit)))
        .service(
            web::resource("/matchmaking/custom-room/{id}/slot")
                .route(web::put().to(custom_room::switch_slot)))
        .service(
            web::resource("/matchmaking/custom-room/{id}/select-archetype/{archetype}")
                .route(web::put().to(custom_room::switch_archetype)))
}