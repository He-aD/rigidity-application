use actix_web::{web, Scope};
use crate::handlers::*;

pub fn get_all() -> Scope {
    web::scope("/api")
}