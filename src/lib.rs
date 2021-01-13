#[macro_use]
extern crate diesel;
extern crate mailgun_rs;
extern crate chrono;

use diesel::{r2d2::ConnectionManager, PgConnection};

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub mod app_conf;
mod handlers;
mod models;
mod services;
mod errors;
mod schema;