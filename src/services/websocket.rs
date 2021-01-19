use actix_web::{web::Data, HttpResponse, Error, HttpRequest, web::Payload};
use actix::Addr;
use actix_web_actors::ws as actix_ws;
use actix::prelude::{Message};

mod ws;
mod lobby;
mod messages;

pub type WebsocketLobby = lobby::Lobby;
#[derive(Message)]
#[rtype(result = "()")]
pub struct ForwardMessage {
    pub id: i32,
    pub message: String,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct MultiForwardMessage {
    pub ids: Vec<i32>,
    pub message: String,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct BroadcastExceptMessage {
    pub ids_to_except: Vec<i32>,
    pub message: String,
}

pub fn new_connection(
    req: HttpRequest, 
    stream: Payload, 
    user_id: i32, 
    srv: Data<Addr<WebsocketLobby>>
) -> Result<HttpResponse, Error> {
    let websocket = ws::WsConn::new(
        user_id,
        srv.get_ref().clone(),
    );
    
    let resp = actix_ws::start(websocket, &req, stream)?;
    Ok(resp)
}