#[macro_use]
extern crate diesel;
extern crate mailgun_rs;
extern crate chrono;

use diesel::{r2d2::ConnectionManager, PgConnection};
use actix::Addr;
use actix::Actor;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub mod app_conf;
pub mod middlewares;
pub mod enums;
mod handlers;
mod models;
mod services;
mod errors;
mod schema;

pub fn new_websocket_lobby() -> Addr<services::websocket::WebsocketLobby> {
    services::websocket::WebsocketLobby::default().start()
}