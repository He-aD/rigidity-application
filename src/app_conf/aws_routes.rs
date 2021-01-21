use actix_web::{web, Scope};
use crate::handlers::aws;

pub fn get_all() -> Scope {
    web::scope("/aws")
        .service(
            web::resource("/sns")
                .route(web::post().to(aws::sns)))
}