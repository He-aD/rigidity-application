use std::collections::HashMap;
use crate::{schema::{custom_room_slots, custom_rooms}};
use crate::enums::{Archetypes, GameModes, Maps};
use crate::handlers::custom_room::{CreateData, SwitchSlotData};
use crate::errors::{AppError, AppResult};
use crate::models::custom_room::{get_without_associations, get_slot_by_position, CustomRoom, CustomRoomSlot};
use diesel::{PgConnection};

#[derive(Insertable)]
#[table_name = "custom_rooms"]
pub struct CustomRoomForm<'a> {
    label: &'a str,
    user_id: &'a i32,
    nb_teams: &'a i32,
    max_player_per_team: &'a i32,
    current_game_mode: &'a GameModes,
    current_map: &'a Maps,
}

impl<'a> CustomRoomForm<'a> {
    pub fn new_from_createdata(create_data: &'a CreateData, user_id: &'a i32) -> Self {
        CustomRoomForm {
            label: &create_data.label,
            user_id: user_id,
            nb_teams: &create_data.nb_teams,
            max_player_per_team: &create_data.max_players_per_team,
            current_game_mode: &create_data.game_mode,
            current_map: &create_data.map,
        }
    }
}

#[derive(Debug, Insertable, AsChangeset)]
#[table_name = "custom_room_slots"]
pub struct CustomRoomSlotForm<'a> {
    custom_room_id: &'a i32,
    team: i32,
    team_position: i32,
    user_id: &'a i32,
    current_archetype: Option<&'a Archetypes>,
}

impl<'a> CustomRoomSlotForm<'a> {
    pub fn new_from_custom_room_creation(
        custom_room_id: &'a i32, 
        user_id: &'a i32,
    ) -> Self {
        CustomRoomSlotForm {
            custom_room_id,
            team: 0,
            team_position: 0,
            user_id,
            current_archetype: Some(&Archetypes::Leader),
        }
    }

    pub fn new_from_user_join(
        custom_room_id: &'a i32, 
        user_id: &'a i32,
        tuple: &'a (CustomRoom, Vec<CustomRoomSlot>)
    ) -> AppResult<Self> {
        let (custom_room, slots) = tuple;
        let room_full_error = Err(AppError::BadRequest(String::from("Can't join, room is full.")));

        if slots.len() < custom_room.get_capacity() {
            let mut empty_slots = HashMap::new();
        
            for i in 0..custom_room.nb_teams {
                let mut hash = HashMap::new();
                for j in 0..custom_room.max_player_per_team {
                    hash.insert(j, j);
                }

                empty_slots.insert(i, hash);
            }
            
            // remove all taken slots from empty_slots
            for slot in slots {
                match empty_slots.get_mut(&slot.team) {
                    Some(slots_of_team) => {
                        slots_of_team.remove(&slot.team_position);
                    }
                    None => {
                        return Err(AppError::InternalServerError(String::from("Custom room error in slot allocation.")))
                    }
                }

            }

            for (team, team_empty_slots) in empty_slots {
                if team_empty_slots.len() > 0 {
                    match team_empty_slots.values().next() {
                        Some(team_position) => {
                            return Ok(CustomRoomSlotForm {
                                custom_room_id,
                                user_id, 
                                team,
                                team_position: *team_position,
                                current_archetype: Some(&Archetypes::Leader),
                            })
                        }, 
                        None => return room_full_error
                    }
                }
            }
            
            return room_full_error
        } else {
            return room_full_error
        }
    }

    pub fn get_custom_room_id(&self) -> i32 {
        self.custom_room_id.clone()
    }

    pub fn new_from_switch_slot(
        custom_room_id: &'a i32,
        user_id: &'a i32,
        slot_data: &SwitchSlotData,
        conn: &PgConnection
    ) -> AppResult<Self> {
        match get_without_associations(custom_room_id, conn) {
            Ok(custom_room) => {
                if !custom_room.is_valid_slot(&slot_data.team, &slot_data.team_position) {
                    return Err(AppError::BadRequest(String::from("Invalid slot position.")))
                }

                match get_slot_by_position(
                    custom_room_id, 
                    &slot_data.team, 
                    &slot_data.team_position,
                    conn) {
                    Ok(_slot) => {
                        Err(AppError::BadRequest(String::from("Slot already taken by someone else")))
                    },
                    Err(_err) => {
                        Ok(CustomRoomSlotForm {
                            custom_room_id: custom_room_id,
                            team: slot_data.team,
                            team_position: slot_data.team_position,
                            user_id: user_id,
                            current_archetype: None,
                        })
                    }
                }
            }, 
            Err(_err) => Err(AppError::BadRequest(format!("Unknown custom room id: {}", custom_room_id)))
        }
        
    }
}