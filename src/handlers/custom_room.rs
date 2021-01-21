use crate::{enums::Archetypes, errors::{AppResult, AppError}};
use actix_web::{HttpResponse, error::{BlockingError}, web, web::Path};
use crate::models::{user, custom_room, custom_room::{CustomRoom, CustomRoomSlot}};
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
            let ws_data = WsData {user_id: &user_id};
            send_multi_forward_message(
                ws,
                &user_id, 
                tuple,
                String::from("quit"),
                conn,
                &ws_data
            )
        },
        Err(err) => Err(AppError::BadRequest(err.to_string()))
    }
}

pub async fn delete(
    id: Identity,
    ws: web::Data<Addr<WebsocketLobby>>,
    pool: web::Data<Pool>
) -> AppResult<HttpResponse> {
    let user_id = id.identity().unwrap();
    match web::block(move || 
        t_delete(
            user_id.parse::<i32>().unwrap(),
            ws,
            pool)).await {
        Ok(_) => {
            Ok(HttpResponse::Ok().finish())
        }
        Err(err) => match err {
            BlockingError::Error(service_error) => Err(service_error),
            BlockingError::Canceled => Err(AppError::InternalServerError(err.to_string())),
        }
    }
}

fn t_delete(
    user_id: i32, 
    ws: web::Data<Addr<WebsocketLobby>>,
    pool: web::Data<Pool>
) -> AppResult<()> {
    let conn = &pool.get().unwrap();

    match custom_room::get_by_user_id(&user_id, conn) {
        Ok(tuple) => {
            if let Err(err) = custom_room::delete(&user_id, conn) {
                return Err(AppError::BadRequest(err.to_string()));
            } 
            #[derive(Serialize)]
            struct Empty{};
            if let Err(err) = send_multi_forward_message(
                ws, 
                &user_id, 
                tuple, 
                String::from("Delete"), 
                conn, 
                &Empty{}) {
                    return Err(AppError::BadRequest(err.to_string()));
                }

            Ok(())
        },
        Err(err) => Err(AppError::BadRequest(err.to_string()))
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
        t_switch_slot(
            custom_room_id,
            user_id.parse::<i32>().unwrap(),
            position,
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

fn t_switch_slot(
    custom_room_id: i32, 
    user_id: i32, 
    position: web::Json<SwitchSlotData>,
    ws: web::Data<Addr<WebsocketLobby>>,
    pool: web::Data<Pool>
) -> AppResult<CustomRoomDto> {
    let conn = &pool.get().unwrap();
    #[derive(Serialize)]
    struct WsData<'a> {
        pub user_id: &'a i32,
        pub nickname: &'a str,
        pub team: &'a i32,
        pub team_position: &'a i32
    }

    let p = position.into_inner();
    let form = CustomRoomSlotForm::new_from_switch_slot(
        &custom_room_id, 
        &user_id,
        &p,
        conn)?;

    match custom_room::update_slot(&user_id, &form, conn) {
        Ok(tuple) => {
            let user = user::get(&user_id, conn).unwrap();
            let ws_data = WsData {
                user_id: &user_id,
                nickname: &user.nickname,
                team: &p.team,
                team_position: &p.team_position
            };
            send_multi_forward_message(
                ws,
                &user_id, 
                tuple,
                String::from("slot"),
                conn,
                &ws_data
            )
        },
        Err(err) => Err(AppError::BadRequest(err.to_string()))
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
        t_switch_archetype(
            custom_room_id,
            archetype,
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

fn t_switch_archetype(
    custom_room_id: i32, 
    archetype: Archetypes,
    user_id: i32, 
    ws: web::Data<Addr<WebsocketLobby>>,
    pool: web::Data<Pool>
) -> AppResult<CustomRoomDto> {
    let conn = &pool.get().unwrap();
    #[derive(Serialize)]
    struct WsData<'a> {
        pub user_id: &'a i32,
        pub archetype: &'a Archetypes,
    }

    match custom_room::update_slot_archetype(
        &user_id, 
        &custom_room_id, 
        &archetype,
        conn) {
        Ok(tuple) => {
            let ws_data = WsData {
                user_id: &user_id,
                archetype: &archetype,
            };
            send_multi_forward_message(
                ws,
                &user_id, 
                tuple,
                String::from("select-archetype"),
                conn,
                &ws_data
            )
        },
        Err(err) => Err(AppError::BadRequest(err.to_string()))
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
        t_kick(
            custom_room_id,
            user_id_to_kick,
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

fn t_kick(
    custom_room_id: i32, 
    user_id_to_kick: i32,
    user_id: i32, 
    ws: web::Data<Addr<WebsocketLobby>>,
    pool: web::Data<Pool>
) -> AppResult<CustomRoomDto> {
    let conn = &pool.get().unwrap();
    match custom_room::delete_slot_by_user_id(&custom_room_id, &user_id_to_kick, conn) {
        Ok(tuple) => {
            #[derive(Serialize)]
            struct WsData {
                pub user_id: i32,
            };
            let data = WsData{user_id: user_id_to_kick};
            match CustomRoomDto::new(tuple, conn) {
                Ok(dto) => {
                    let mut user_ids = dto.get_all_user_ids_except(&user_id);
                    user_ids.push(user_id_to_kick);
                    let msg = MultiForwardMessage::new(
                        &user_ids,
                        ServerMessage::new(
                            String::from("/matchmaking/custom-room"),
                            String::from("kick"),
                            &data)
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

pub async fn start_matchmaking(
    Path(custom_room_id): Path<i32>,
    id: Identity,
    ws: web::Data<Addr<WebsocketLobby>>,
    pool: web::Data<Pool>
) -> AppResult<HttpResponse> {
    let user_id = id.identity().unwrap();
    match web::block(move || 
        t_start_matchmaking(
            custom_room_id,
            user_id.parse::<i32>().unwrap(),
            ws,
            pool)).await {
        Ok(_custom_room) => {
            Ok(HttpResponse::Ok().finish())
        }
        Err(err) => match err {
            BlockingError::Error(service_error) => Err(service_error),
            BlockingError::Canceled => Err(AppError::InternalServerError(err.to_string())),
        }
    }
}

fn t_start_matchmaking(
    custom_room_id: i32,
    user_id: i32, 
    ws: web::Data<Addr<WebsocketLobby>>,
    pool: web::Data<Pool>
) -> AppResult<()> {
    let conn = &pool.get().unwrap();
    match custom_room::get(&custom_room_id, conn) {
        Ok(tuple) => {
            // call AWS matchmaking request here

            #[derive(Serialize)]
            struct Empty{};
            let data = &Empty{};
            if let Err(err) = send_multi_forward_message(
                ws, 
                &user_id, 
                tuple, 
                String::from("start-matchmaking"), 
                conn, 
                data) {
                    return Err(AppError::BadRequest(err.to_string()));
                }
            
            Ok(())
        },
        Err(err) => Err(AppError::BadRequest(err.to_string()))
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
        t_stop_matchmaking(
            custom_room_id,
            user_id.parse::<i32>().unwrap(),
            ws,
            pool)).await {
        Ok(_custom_room) => {
            Ok(HttpResponse::Ok().finish())
        }
        Err(err) => match err {
            BlockingError::Error(service_error) => Err(service_error),
            BlockingError::Canceled => Err(AppError::InternalServerError(err.to_string())),
        }
    }
}

fn t_stop_matchmaking(
    custom_room_id: i32,
    user_id: i32, 
    ws: web::Data<Addr<WebsocketLobby>>,
    pool: web::Data<Pool>
) -> AppResult<()> {
    let conn = &pool.get().unwrap();
    match custom_room::get(&custom_room_id, conn) {
        Ok(tuple) => {
            // call AWS matchmaking request here

            #[derive(Serialize)]
            struct Empty{};
            let data = &Empty{};
            if let Err(err) = send_multi_forward_message(
                ws, 
                &user_id, 
                tuple, 
                String::from("stop-matchmaking"), 
                conn, 
                data) {
                    return Err(AppError::BadRequest(err.to_string()));
                }
            
            Ok(())
        },
        Err(err) => Err(AppError::BadRequest(err.to_string()))
    }
}

fn send_multi_forward_message<T: Serialize>(
    ws: web::Data<Addr<WebsocketLobby>>,
    user_id: &i32,
    tuple: (CustomRoom, Vec<CustomRoomSlot>), 
    typ: String,
    conn: &diesel::PgConnection,
    data: &T) -> AppResult<CustomRoomDto> {
    match CustomRoomDto::new(tuple, conn) {
        Ok(dto) => {
            let user_ids = dto.get_all_user_ids_except(&user_id);
            let msg = MultiForwardMessage::new(
                &user_ids,
                ServerMessage::new(
                    String::from("/matchmaking/custom-room"),
                    typ,
                    data)
            );
            let _ = ws.get_ref().do_send(msg);

            Ok(dto)
        },
        Err(err) => Err(AppError::BadRequest(err.to_string()))
    }
}