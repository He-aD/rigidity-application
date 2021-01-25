use serde::{Serialize};
use crate::models::custom_room::{CustomRoomSlot, CustomRoom};
use crate::enums::{GameModes, Maps, Archetypes};
use crate::models::user;
use diesel::{PgConnection};
use crate::models::ORMResult;
use uuid::Uuid;

#[derive(Serialize)]
pub struct CustomRoomDto {
    pub id: i32,
    pub label: String,
    pub user_id: i32,
    pub nb_teams: i32,
    pub max_player_per_team: i32,
    pub game_mode: GameModes,
    pub map: Maps,
    pub matchmaking_ticket: Option<Uuid>,
    pub slots: Vec<CustomRoomSlotDto>
}

impl CustomRoomDto {
    pub fn new(tuple: (CustomRoom, Vec<CustomRoomSlot>), conn: &PgConnection) -> ORMResult<Self> {
        let mut slots = Vec::new();

        for slot in tuple.1 {
            slots.push(CustomRoomSlotDto::new(slot, conn)?);
        }
        
        Ok(CustomRoomDto {
            id: tuple.0.id,
            label: tuple.0.label,
            user_id: tuple.0.user_id,
            nb_teams: tuple.0.nb_teams,
            max_player_per_team: tuple.0.max_player_per_team,
            game_mode: tuple.0.current_game_mode,
            map: tuple.0.current_map,
            matchmaking_ticket: tuple.0.matchmaking_ticket,
            slots: slots,
        })
    }

    pub fn get_all_user_ids_except(&self, except_id: &i32) -> Vec<i32> {
        let mut result = Vec::new();

        for slot in &self.slots {
            if slot.user_id != *except_id {
                result.push(slot.user_id.clone());
            }
        }

        result
    }

    pub fn get_all_user_ids(&self) -> Vec<i32> {
        let mut result = Vec::new();

        for slot in &self.slots {
            result.push(slot.user_id.clone());
        }

        result
    }

    pub fn get_slot_index_from_user_id(&self, user_id: &i32) -> Option<usize> {
        let mut i = 0;
        for slot in &self.slots {
            if slot.user_id == *user_id {
                let j = i as usize;
                return Some(j);
            }
            i += 1;
        }

        None
    }
}

#[derive(Serialize)]
pub struct CustomRoomSlotDto {
    pub id: i32,
    pub custom_room_id: i32,
    pub team: i32,
    pub team_position: i32,
    pub user_id: i32,
    pub nickname: String,
    pub archetype: Archetypes,
}

impl CustomRoomSlotDto {
    pub fn new(slot: CustomRoomSlot, conn: &PgConnection) -> ORMResult<Self> {
        let user = user::get(&slot.user_id, conn)?;

        Ok (CustomRoomSlotDto {
            id: slot.id,
            custom_room_id: slot.custom_room_id,
            team: slot.team,
            team_position: slot.team_position,
            user_id: slot.user_id,
            nickname: user.nickname,
            archetype: slot.current_archetype,
        })
    }
}