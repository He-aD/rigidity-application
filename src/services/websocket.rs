use actix_web::{web::Data, HttpResponse, Error, HttpRequest, web::Payload};
use actix::Addr;
use actix_web_actors::ws as actix_ws;

mod ws;
mod lobby;
mod messages;

pub type WebsocketLobby = lobby::Lobby;

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