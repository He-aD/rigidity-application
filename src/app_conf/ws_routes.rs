use actix_web::{web, Route};
use crate::handlers;

pub fn get() -> Route {
    web::get().to(handlers::new_websocket)
}