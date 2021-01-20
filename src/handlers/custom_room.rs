use crate::errors::{AppResult, AppError};
use actix_web::{HttpResponse, error::{BlockingError}, web, web::Path};
use crate::models::{custom_room};
use crate::models::custom_room::form::{CustomRoomSlotForm};
use crate::enums::{Maps, GameModes};
use serde::{Deserialize, Serialize};
use crate::Pool;
use actix_identity::Identity;
use crate::services::websocket::{ServerMessage, BroadcastExceptMessage, WebsocketLobby, MultiForwardMessage};
use actix::{Addr};
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
                    let msg = BroadcastExceptMessage::new(
                        &vec![user_id],
                        ServerMessage::new(
                            String::from("/matchmaking/custom-room"),
                            String::from("new"),
                            &dto
                        )
                    );
                    
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

pub async fn join(
    Path(custom_room_id): Path<i32>,
    id: Identity,
    ws: web::Data<Addr<WebsocketLobby>>,
    pool: web::Data<Pool>
) -> AppResult<HttpResponse> {
    let user_id = id.identity().unwrap();
    match web::block(move || 
        t_join(
            custom_room_id,
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

fn t_join(
    custom_room_id: i32, 
    user_id: i32, 
    ws: web::Data<Addr<WebsocketLobby>>,
    pool: web::Data<Pool>
) -> AppResult<CustomRoomDto> {
    let conn = &pool.get().unwrap();

    match custom_room::get(&custom_room_id, conn) {
        Ok(tuple) => {
            let form = CustomRoomSlotForm::new_from_user_join(&custom_room_id, &user_id, &tuple)?;
            match custom_room::create_slot(&form, conn) {
                Ok(tuple) => {
                    match CustomRoomDto::new(tuple, &conn) {
                        Ok(dto) => {
                            let user_ids = dto.get_all_user_ids_except(&user_id);
                            if let Some(slot_dto_index) = dto.get_slot_index_from_user_id(&user_id) {
                                let msg = MultiForwardMessage::new(
                                    &user_ids,
                                    ServerMessage::new(
                                        String::from("/matchmaking/custom-room"),
                                        String::from("join"),
                                        &dto.slots.get(slot_dto_index))
                                );

                                let _ = ws.get_ref().do_send(msg);
                            } else {
                                return Err(AppError::InternalServerError(String::from("Error in Custom room dtos."))) 
                            }
                            return Ok(dto);
                        }, 
                        Err(err) => {
                            Err(AppError::BadRequest(err.to_string())) 
                        }
                    }

                },
                Err(err) => Err(AppError::BadRequest(err.to_string()))
            }
        },
        Err(err) => {
            Err(AppError::BadRequest(err.to_string()))
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
        t_quit(
            custom_room_id,
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

fn t_quit(
    custom_room_id: i32, 
    user_id: i32, 
    ws: web::Data<Addr<WebsocketLobby>>,
    pool: web::Data<Pool>
) -> AppResult<CustomRoomDto> {
    let conn = &pool.get().unwrap();
    #[derive(Serialize)]
    struct WsData<'a> {
        pub user_id: &'a i32
    }

    match custom_room::delete_slot_by_user_id(&custom_room_id, &user_id, conn) {
        Ok(tuple) => {
            match CustomRoomDto::new(tuple, conn) {
                Ok(dto) => {
                    let user_ids = dto.get_all_user_ids_except(&user_id);
                    let ws_data = WsData {user_id: &user_id};
                    let msg = MultiForwardMessage::new(
                        &user_ids,
                        ServerMessage::new(
                            String::from("/matchmaking/custom-room"),
                            String::from("quit"),
                            &ws_data)
                    );
                    let _ = ws.get_ref().do_send(msg);

                    Ok(dto)
                },
                Err(err) => Err(AppError::BadRequest(err.to_string()))
            }
        },
        Err(err) => Err(AppError::BadRequest(err.to_string()))
    }
}