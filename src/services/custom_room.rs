use crate::models::{user, custom_room, custom_room::{CustomRoom, CustomRoomSlot}};
use actix::{Addr};
use rusoto_gamelift::*;
use crate::services::websocket::{ServerMessage, BroadcastExceptMessage, WebsocketLobby, MultiForwardMessage, ForwardMessage};
use crate::models::custom_room::form::{CustomRoomSlotForm};
use serde::{Serialize};
use crate::handlers::custom_room::dtos::CustomRoomDto;
use crate::handlers::custom_room::{CreateData, SwitchSlotData};
use crate::errors::{AppResult, AppError};
use crate::enums::Archetypes;
use diesel::{PgConnection};
use uuid::Uuid;
use crate::services::aws::{FlexMatchEvents, FlexMatchData, FlexMatchSucceededDetail};

pub fn get_all(
    conn: &PgConnection
) -> AppResult<Vec<CustomRoomDto>> {
    let mut results = Vec::new();
    match custom_room::get_all(conn) {
        Ok(custom_rooms) => {
            for tuple in custom_rooms {
                match CustomRoomDto::new(tuple, conn) {
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

pub fn create(    
    create_data: CreateData,
    user_id: i32,
    ws: Addr<WebsocketLobby>,
    conn: &PgConnection
) -> AppResult<CustomRoomDto> {
    match custom_room::create(&user_id, create_data, conn) {
        Ok(tuple) => {
            match CustomRoomDto::new(tuple, conn) {
                Ok(dto) => {
                    let msg = BroadcastExceptMessage::new(
                        &vec![user_id],
                        ServerMessage::new(
                            String::from("/matchmaking/custom-room"),
                            String::from("new"),
                            &dto
                        )
                    );
                    
                    let _ = ws.do_send(msg);
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

pub fn join(
    custom_room_id: i32, 
    user_id: i32, 
    ws: Addr<WebsocketLobby>,
    conn: &PgConnection
) -> AppResult<CustomRoomDto> {
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

                                let _ = ws.do_send(msg);
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

pub fn quit(
    custom_room_id: i32, 
    user_id: i32, 
    ws: Addr<WebsocketLobby>,
    conn: &PgConnection
) -> AppResult<CustomRoomDto> {
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

pub fn delete(
    user_id: i32, 
    ws: Addr<WebsocketLobby>,
    conn: &PgConnection
) -> AppResult<()> {
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
                String::from("delete"), 
                conn, 
                &Empty{}) {
                    return Err(AppError::BadRequest(err.to_string()));
                }

            Ok(())
        },
        Err(err) => Err(AppError::BadRequest(err.to_string()))
    }
}

pub fn switch_slot(
    custom_room_id: i32, 
    user_id: i32, 
    position: SwitchSlotData,
    ws: Addr<WebsocketLobby>,
    conn: &PgConnection
) -> AppResult<CustomRoomDto> {
    #[derive(Serialize)]
    struct WsData<'a> {
        pub user_id: &'a i32,
        pub nickname: &'a str,
        pub team: &'a i32,
        pub team_position: &'a i32
    }

    let form = CustomRoomSlotForm::new_from_switch_slot(
        &custom_room_id, 
        &user_id,
        &position,
        conn)?;

    match custom_room::update_slot(&user_id, &form, conn) {
        Ok(tuple) => {
            let user = user::get(&user_id, conn).unwrap();
            let ws_data = WsData {
                user_id: &user_id,
                nickname: &user.nickname,
                team: &position.team,
                team_position: &position.team_position
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

pub fn switch_archetype(
    custom_room_id: i32, 
    archetype: Archetypes,
    user_id: i32, 
    ws: Addr<WebsocketLobby>,
    conn: &PgConnection
) -> AppResult<CustomRoomDto> {
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

pub fn kick(
    custom_room_id: i32, 
    user_id_to_kick: i32,
    o_user_id: Option<i32>, 
    ws: Addr<WebsocketLobby>,
    conn: &PgConnection
) -> AppResult<CustomRoomDto> {
    match custom_room::delete_slot_by_user_id(&custom_room_id, &user_id_to_kick, conn) {
        Ok(tuple) => {
            #[derive(Serialize)]
            struct WsData {
                pub user_id: i32,
            };
            let data = WsData{user_id: user_id_to_kick};
            match CustomRoomDto::new(tuple, conn) {
                Ok(dto) => {
                    let mut user_ids ;
                    let message;
                    if let Some(user_id) = o_user_id {
                        user_ids = dto.get_all_user_ids_except(&user_id);
                        user_ids.push(user_id_to_kick);
                        message = String::from("kick");
                    } else { // disconnect
                        user_ids = dto.get_all_user_ids();
                        message = String::from("disconnect");
                    }

                    let msg = MultiForwardMessage::new(
                        &user_ids,
                        ServerMessage::new(
                            String::from("/matchmaking/custom-room"),
                            message,
                            &data)
                    );
                    let _ = ws.do_send(msg);
        
                    Ok(dto)
                },
                Err(err) => Err(AppError::BadRequest(err.to_string()))
            }
        },
        Err(err) => Err(AppError::BadRequest(err.to_string()))
    }
}

pub async fn start_matchmaking(
    custom_room_id: i32,
    user_id: i32, 
    ws: Addr<WebsocketLobby>,
    gamelift: &GameLiftClient,
    conn: &PgConnection
) -> AppResult<()> {
    match custom_room::get_with_users(&custom_room_id, conn) {
        Ok((custom_room, tuples)) => {
            if custom_room.user_id != user_id {
                return Err(AppError::BadRequest(String::from("Only the room owner can start matchmaking.")))
            }

            let ticket_id = Uuid::new_v4();
            let start_matchmaking_input = custom_room.get_start_matchmaking_input(&tuples, &ticket_id);
            println!("{:?}", start_matchmaking_input);
            match gamelift.start_matchmaking(start_matchmaking_input).await {
                Ok(result) => {
                    if let Some(_matchmaking_ticket) = result.matchmaking_ticket {                                               
                        if let Err(err) = custom_room::update_ticket(
                            &custom_room_id, 
                            &Some(ticket_id), 
                            conn) {
                            return Err(AppError::InternalServerError(err.to_string()));
                        }
    
                        #[derive(Serialize)]
                        struct Empty{};
                        let data = &Empty{};
                        let mut slots = Vec::new();
                        for (slot, _user) in tuples {
                            slots.push(slot);
                        }
                        if let Err(err) = send_multi_forward_message(
                            ws, 
                            &user_id, 
                            (custom_room, slots), 
                            String::from("start-matchmaking"), 
                            conn, 
                            data) {
                            return Err(AppError::BadRequest(err.to_string()));
                        }
                        
                        return Ok(())
                    }
                    
                    Err(AppError::InternalServerError(String::from("Problem with aws matchmaking.")))
                }, 
                Err(err) => {
                    Err(AppError::BadRequest(err.to_string()))
                }
            }
        },
        Err(err) => Err(AppError::BadRequest(err.to_string()))
    }
}

pub async fn stop_matchmaking(
    custom_room_id: i32,
    user_id: i32, 
    ws: Addr<WebsocketLobby>,
    gamelift: &GameLiftClient,
    conn: &PgConnection
) -> AppResult<()> {
    match custom_room::get(&custom_room_id, conn) {
        Ok(tuple) => {
            if tuple.0.user_id != user_id {
                return Err(AppError::BadRequest(String::from("Only the room owner can stop matchmaking.")))
            }
            if tuple.0.matchmaking_ticket == None {
                return Err(AppError::BadRequest(String::from("No matchmaking started for this room.")))
            }

            match gamelift.stop_matchmaking(StopMatchmakingInput {
                ticket_id: tuple.0.matchmaking_ticket.unwrap().to_string()
            }).await {
                Ok(_result) => {
                    if let Err(err) = custom_room::update_ticket(
                        &custom_room_id, 
                        &None, 
                        conn) {
                        return Err(AppError::InternalServerError(err.to_string()));
                    }
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
                Err(err) => {
                    Err(AppError::BadRequest(err.to_string()))
                }
            }
        },
        Err(err) => Err(AppError::BadRequest(err.to_string()))
    }
}

pub fn matchmaking_succeeded(
    data: FlexMatchData<FlexMatchSucceededDetail>,
    ws: Addr<WebsocketLobby>,
    conn: &PgConnection
) -> AppResult<()> {
    let ticket_id = Uuid::parse_str(&data.detail.tickets[0].ticket_id).unwrap();
    
    match custom_room::get_by_ticket_id(ticket_id, conn) {
        Ok((custom_room, slots)) => {
            #[derive(Serialize)]
            struct WsData<'a> {
                pub ip_address: &'a str,
                pub port: &'a i32,
                pub player_id: &'a str,
                pub player_session_id: &'a str
            }
            
            for slot in slots {
                let str_user_id = slot.user_id.to_string();
                for player in &data.detail.game_session_info.players {
                    if player.player_id == str_user_id {
                        let ws_data = WsData {
                            ip_address: &data.detail.game_session_info.ip_address,
                            port: &data.detail.game_session_info.port,
                            player_id: &player.player_id,
                            player_session_id: &player.player_session_id
                        };

                        let msg = ForwardMessage::new(
                            &slot.user_id,
                            ServerMessage::new(
                                String::from("/matchmaking/custom-room"),
                                String::from("matchmaking-succeeded"),
                                &ws_data)
                        );
                        let _ = ws.do_send(msg);   
                        break;
                    }        
                }
            }
                
            if let Err(err) = custom_room::delete(&custom_room.user_id, conn) {
                return Err(AppError::InternalServerError(err.to_string()))
            }
        },
        Err(err) => {
            return Err(AppError::InternalServerError(err.to_string()))
        }
    }

    Ok(())
}

pub async fn matchmaking_failed(
    reason: FlexMatchEvents,
    ticket_id: &str,
    ws: Addr<WebsocketLobby>,
    conn: &PgConnection
) -> AppResult<()> {
    let uuid_ticket_id = Uuid::parse_str(ticket_id).unwrap();
    match custom_room::get_by_ticket_id(uuid_ticket_id, conn) {
        Ok((custom_room, slots)) => {
            #[derive(Serialize)]
            struct WsData {
                pub reason: String
            }

            let mut user_ids = Vec::new();
            for slot in slots {
                user_ids.push(slot.user_id);
            }

            let msg = MultiForwardMessage::new(
                &user_ids,
                ServerMessage::new(
                    String::from("/matchmaking/custom-room"),
                    String::from("matchmaking-failed"),
                    &WsData {reason: reason.to_string()})
            );
            let _ = ws.do_send(msg); 
            
            if let Err(err) = custom_room::update_ticket(&custom_room.id, &None, conn) {
                return Err(AppError::InternalServerError(err.to_string()))
            }
        },
        Err(err) => {
            return Err(AppError::InternalServerError(err.to_string()))
        }
    }

    Ok(())
}

pub fn handle_websocket_closing(
    user_id: &i32, 
    ws: Addr<WebsocketLobby>,
    conn: &PgConnection
) {
    if let Ok(slot) = custom_room::get_slot_by_user_id(user_id, conn) {
        match custom_room::get(&slot.custom_room_id, conn) {
            Ok(tuple) => {
                if tuple.0.user_id == *user_id {
                    if let Err(_err) = delete(*user_id, ws, conn) {
                        // futur logger service InternalServerError
                    }
                } else {
                    if let Err(_err) = kick(
                        slot.custom_room_id, 
                        *user_id, 
                        None, 
                        ws,
                        conn) {
                        // futur logger service InternalServerError
                    }
                }
            },
            Err(_) => {
                // futur logger service InternalServerError
            }
        }
    }
}

fn send_multi_forward_message<T: Serialize>(
    ws: Addr<WebsocketLobby>,
    user_id: &i32,
    tuple: (CustomRoom, Vec<CustomRoomSlot>), 
    typ: String,
    conn: &diesel::PgConnection,
    data: &T
) -> AppResult<CustomRoomDto> {
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
            let _ = ws.do_send(msg);

            Ok(dto)
        },
        Err(err) => Err(AppError::BadRequest(err.to_string()))
    }
}