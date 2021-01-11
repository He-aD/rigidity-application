use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use actix_web::{middleware};
use actix_identity::{CookieIdentityPolicy, IdentityService};
use time::Duration;
use super::{models, utils};

pub mod static_routes;
pub mod api_routes;


#[cfg(debug_assertions)]
pub fn middleware_logger() -> middleware::Logger {
    middleware::Logger::default()
}

#[cfg(debug_assertions)]
pub fn middleware_identity_service() -> IdentityService<CookieIdentityPolicy> {
    IdentityService::new(
        CookieIdentityPolicy::new(utils::SECRET_KEY.as_bytes())
            .name("auth")
            .path("/api")
            .domain(get_domain().as_str())
            .max_age_time(Duration::days(1))
            .secure(false), // this can only be true if you have https
    )
}

#[cfg(debug_assertions)]
pub fn set_env() {
    dotenv::dotenv().ok();
    std::env::set_var(
        "RUST_LOG",
        "simple-auth-server=debug,actix_web=info,actix_server=info",
    );
    env_logger::init();
}

#[cfg(debug_assertions)]
pub fn connect_database() -> models::Pool {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    // create db connection pool
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}

#[cfg(debug_assertions)]
fn get_domain() -> String {
    std::env::var("DOMAIN").unwrap_or_else(|_| "localhost".to_string())
}

// #[cfg(not(debug_assertions))]
// pub fn middleware_logger() -> middleware::Logger {
//     TODO: implement
// }

// #[cfg(not(debug_assertions))]
// pub fn middleware_identity_service() -> IdentityService<CookieIdentityPolicy> {
//     TODO: implement
// }

// #[cfg(not(debug_assertions))]
// fn set_env() {
//     TODO: implement
// }

// #[cfg(not(debug_assertions))]
// fn connect_database() -> models::Pool {
//     TODO: implement
// }

// #[cfg(not(debug_assertions))]
// fn get_domain() -> String {
//     TODO: implement
// }

