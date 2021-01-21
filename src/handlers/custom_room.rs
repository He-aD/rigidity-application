use crate::{enums::Archetypes, errors::{AppResult, AppError}};
use actix_web::{HttpResponse, error::{BlockingError}, web, web::Path};
use crate::enums::{Maps, GameModes};
use serde::{Deserialize};
use crate::Pool;
use actix_identity::Identity;
use crate::services::{custom_room as service, websocket::WebsocketLobby};
use actix::{Addr};

pub mod dtos;

pub async fn get_all(
    pool: web::Data<Pool>
) -> AppResult<HttpResponse> {    
    match web::block(move || 
        service::get_all(&pool.get().unwrap())).await {
        Ok(custom_rooms) => {
            Ok(HttpResponse::Ok().json(custom_rooms))
        }
        Err(err) => match err {
            BlockingError::Error(service_error) => Err(service_error),
            BlockingError::Canceled => Err(AppError::InternalServerError(err.to_string())),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateData {
    pub label: String,
    pub nb_teams: i32,
    pub max_players_per_team: i32,
    pub game_mode: GameModes,
    pub map: Maps
}

pub async fn create(
    create_data: web::Json<CreateData>,
    id: Identity,
    ws: web::Data<Addr<WebsocketLobby>>,
    pool: web::Data<Pool>
) -> AppResult<HttpResponse> {
    let user_id = id.identity().unwrap();
    match web::block(move || 
        service::create(
            create_data.into_inner(),
            user_id.parse::<i32>().unwrap(),
            ws.get_ref().to_owned(),
            &pool.get().unwrap())).await {
        Ok(custom_room) => {
            Ok(HttpResponse::Ok().json(custom_room))
        }
        Err(err) => match err {
            BlockingError::Error(service_error) => Err(service_error),
            BlockingError::Canceled => Err(AppError::InternalServerError(err.to_string())),
        }
    }
}

pub async fn join(
    Path(custom_room_id): Path<i32>,
    id: Identity,
    ws: web::Data<Addr<WebsocketLobby>>,
    pool: web::Data<Pool>
) -> AppResult<HttpResponse> {
    let user_id = id.identity().unwrap();
    match web::block(move || 
        service::join(
            custom_room_id,
            user_id.parse::<i32>().unwrap(),
            ws.get_ref().to_owned(),
            &pool.get().unwrap())).await {
        Ok(custom_room) => {
            Ok(HttpResponse::Ok().json(custom_room))
        }
        Err(err) => match err {
            BlockingError::Error(service_error) => Err(service_error),
            BlockingError::Canceled => Err(AppError::InternalServerError(err.to_string())),
        }
    }
}

pub async fn quit(
    Path(custom_room_id): Path<i32>,
    id: Identity,
    ws: web::Data<Addr<WebsocketLobby>>,
    pool: web::Data<Pool>
) -> AppResult<HttpResponse> {
    let user_id = id.identity().unwrap();
    match web::block(move || 
        service::quit(
            custom_room_id,
            user_id.parse::<i32>().unwrap(),
            ws.get_ref().to_owned(),
            &pool.get().unwrap())).await {
        Ok(custom_room) => {
            Ok(HttpResponse::Ok().json(custom_room))
        }
        Err(err) => match err {
            BlockingError::Error(service_error) => Err(service_error),
            BlockingError::Canceled => Err(AppError::InternalServerError(err.to_string())),
        }
    }
}

pub async fn delete(
    id: Identity,
    ws: web::Data<Addr<WebsocketLobby>>,
    pool: web::Data<Pool>
) -> AppResult<HttpResponse> {
    let user_id = id.identity().unwrap();
    match web::block(move || 
        service::delete(
            user_id.parse::<i32>().unwrap(),
            ws.get_ref().to_owned(),
            &pool.get().unwrap())).await {
        Ok(_) => {
            Ok(HttpResponse::Ok().finish())
        }
        Err(err) => match err {
            BlockingError::Error(service_error) => Err(service_error),
            BlockingError::Canceled => Err(AppError::InternalServerError(err.to_string())),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct SwitchSlotData {
    pub team: i32,
    pub team_position: i32,
}

pub async fn switch_slot(
    Path(custom_room_id): Path<i32>,
    id: Identity,
    position: web::Json<SwitchSlotData>,
    ws: web::Data<Addr<WebsocketLobby>>,
    pool: web::Data<Pool>
) -> AppResult<HttpResponse> {
    let user_id = id.identity().unwrap();
    match web::block(move || 
        service::switch_slot(
            custom_room_id,
            user_id.parse::<i32>().unwrap(),
            position.into_inner(),
            ws.get_ref().to_owned(),
            &pool.get().unwrap())).await {
        Ok(custom_room) => {
            Ok(HttpResponse::Ok().json(custom_room))
        }
        Err(err) => match err {
            BlockingError::Error(service_error) => Err(service_error),
            BlockingError::Canceled => Err(AppError::InternalServerError(err.to_string())),
        }
    }
}

pub async fn switch_archetype(
    param: Path<(i32, Archetypes)>,
    id: Identity,
    ws: web::Data<Addr<WebsocketLobby>>,
    pool: web::Data<Pool>
) -> AppResult<HttpResponse> {
    let user_id = id.identity().unwrap();
    let (custom_room_id, archetype) = param.into_inner();
    match web::block(move || 
        service::switch_archetype(
            custom_room_id,
            archetype,
            user_id.parse::<i32>().unwrap(),
            ws.get_ref().to_owned(),
            &pool.get().unwrap())).await {
        Ok(custom_room) => {
            Ok(HttpResponse::Ok().json(custom_room))
        }
        Err(err) => match err {
            BlockingError::Error(service_error) => Err(service_error),
            BlockingError::Canceled => Err(AppError::InternalServerError(err.to_string())),
        }
    }
}

pub async fn kick(
    param: Path<(i32, i32)>,
    id: Identity,
    ws: web::Data<Addr<WebsocketLobby>>,
    pool: web::Data<Pool>
) -> AppResult<HttpResponse> {
    let user_id = id.identity().unwrap();
    let (custom_room_id, user_id_to_kick) = param.into_inner();
    match web::block(move || 
        service::kick(
            custom_room_id,
            user_id_to_kick,
            Some(user_id.parse::<i32>().unwrap()),
            ws.get_ref().to_owned(),
            &pool.get().unwrap())).await {
        Ok(custom_room) => {
            Ok(HttpResponse::Ok().json(custom_room))
        }
        Err(err) => match err {
            BlockingError::Error(service_error) => Err(service_error),
            BlockingError::Canceled => Err(AppError::InternalServerError(err.to_string())),
        }
    }
}

pub async fn start_matchmaking(
    Path(custom_room_id): Path<i32>,
    id: Identity,
    ws: web::Data<Addr<WebsocketLobby>>,
    pool: web::Data<Pool>
) -> AppResult<HttpResponse> {
    let user_id = id.identity().unwrap();
    match web::block(move || 
        service::start_matchmaking(
            custom_room_id,
            user_id.parse::<i32>().unwrap(),
            ws.get_ref().to_owned(),
            &pool.get().unwrap())).await {
        Ok(_custom_room) => {
            Ok(HttpResponse::Ok().finish())
        }
        Err(err) => match err {
            BlockingError::Error(service_error) => Err(service_error),
            BlockingError::Canceled => Err(AppError::InternalServerError(err.to_string())),
        }
    }
}

pub async fn stop_matchmaking(
    Path(custom_room_id): Path<i32>,
    id: Identity,
    ws: web::Data<Addr<WebsocketLobby>>,
    pool: web::Data<Pool>
) -> AppResult<HttpResponse> {
    let user_id = id.identity().unwrap();
    match web::block(move || 
        service::stop_matchmaking(
            custom_room_id,
            user_id.parse::<i32>().unwrap(),
            ws.get_ref().to_owned(),
            &pool.get().unwrap())).await {
        Ok(_custom_room) => {
            Ok(HttpResponse::Ok().finish())
        }
        Err(err) => match err {
            BlockingError::Error(service_error) => Err(service_error),
            BlockingError::Canceled => Err(AppError::InternalServerError(err.to_string())),
        }
    }
}