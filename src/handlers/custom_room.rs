use crate::errors::{AppResult, AppError};
use actix_web::{web, error::BlockingError, HttpResponse};
use crate::models::{custom_room};
use crate::enums::{Maps, GameModes};
use serde::{Deserialize};
use crate::Pool;
use actix_identity::Identity;
use crate::services::websocket::{BroadcastExceptMessage, WebsocketLobby};
use actix::Addr;
use dtos::CustomRoomDto;

mod dtos;

pub async fn get_all(
    pool: web::Data<Pool>
) -> AppResult<HttpResponse> {    
    match web::block(move || 
        t_get_all(pool)).await {
        Ok(custom_rooms) => {
            Ok(HttpResponse::Ok().json(custom_rooms))
        }
        Err(err) => match err {
            BlockingError::Error(service_error) => Err(service_error),
            BlockingError::Canceled => Err(AppError::InternalServerError(err.to_string())),
        }
    }
}

fn t_get_all(
    pool: web::Data<Pool>
) -> AppResult<Vec<CustomRoomDto>> {
    let mut results = Vec::new();
    match custom_room::get_all(&pool.get().unwrap()) {
        Ok(custom_rooms) => {
            for tuple in custom_rooms {
                match CustomRoomDto::new(tuple, &pool.get().unwrap()) {
                    Ok(vector) => results.push(vector),
                    Err(err) => return Err(AppError::InternalServerError(err.to_string()))
                }
            }

            Ok(results)
        }
        Err(err) => {
            Err(AppError::BadRequest(err.to_string()))
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
        t_create(
            create_data,
            user_id.parse::<i32>().unwrap(),
            ws,
            pool)).await {
        Ok(custom_room) => {
            Ok(HttpResponse::Ok().json(custom_room))
        }
        Err(err) => match err {
            BlockingError::Error(service_error) => Err(service_error),
            BlockingError::Canceled => Err(AppError::InternalServerError(err.to_string())),
        }
    }
}

fn t_create(    
    create_data: web::Json<CreateData>,
    user_id: i32,
    ws: web::Data<Addr<WebsocketLobby>>,
    pool: web::Data<Pool>
) -> AppResult<CustomRoomDto> {
    match custom_room::create(&user_id, create_data.into_inner(), &pool.get().unwrap()) {
        Ok(tuple) => {
            match CustomRoomDto::new(tuple, &pool.get().unwrap()) {
                Ok(dto) => {
                    let msg = BroadcastExceptMessage {
                        ids_to_except: vec![user_id],
                        message: serde_json::to_string(&dto).unwrap()
                    };
                    
                    let _ = ws.get_ref().do_send(msg);
                    Ok(dto)
                },
                Err(err) => return Err(AppError::InternalServerError(err.to_string()))
            }
        }
        Err(err) => {
            Err(AppError::BadRequest(err.to_string()))
        }
    }
}