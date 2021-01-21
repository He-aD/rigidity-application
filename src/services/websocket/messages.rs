use actix::prelude::Message;
use actix::Addr;
use super::ws::WsConn;
use super::lobby::Lobby;

//WsConn responds to this to pipe it through to the actual client
#[derive(Message)]
#[rtype(result = "()")]
pub struct WsMessage(pub String);

//WsConn sends this to the lobby to say "put me in please"
#[derive(Message)]
#[rtype(result = "()")]
pub struct Connect {
    pub addr: Addr<WsConn>,
    pub self_id: i32,
}

//WsConn sends this to a lobby to say "take me out please"
#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub addr: Addr<Lobby>, 
    pub id: i32,
}