use actix_identity::Identity;
use actix_web::{web::Data, web::Payload, HttpResponse, HttpRequest};
use crate::errors::*;
use crate::services::websocket::{new_connection, WebsocketLobby};
use actix::Addr;

pub mod auth;
pub mod custom_room;
pub mod aws;

pub async fn new_websocket(
    req: HttpRequest,
    stream: Payload,
    id: Identity,
    srv: Data<Addr<WebsocketLobby>>
) -> AppResult<HttpResponse> {    
    if let Some(user_id) = id.identity() {
        match new_connection(
            req, 
            stream, 
            user_id.parse::<i32>().unwrap(), 
            srv) {
            Ok(resp) => {
                return Ok(resp);
            }
            Err(err) => {
                return Err(AppError::InternalServerError(err.to_string()));
            }
        }
    }

    Err(AppError::Unauthorized)
}