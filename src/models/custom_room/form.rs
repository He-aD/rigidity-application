use crate::{schema::{custom_room_slots, custom_rooms}};
use crate::enums::{Archetypes, GameModes, Maps};
use crate::handlers::custom_room::{CreateData};

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

#[derive(Insertable)]
#[table_name = "custom_room_slots"]
pub struct CustomRoomSlotForm<'a> {
    custom_room_id: &'a i32,
    team: &'a i32,
    team_position: &'a i32,
    user_id: &'a i32,
    current_archetype: &'a Archetypes,
}

impl<'a> CustomRoomSlotForm<'a> {
    pub fn new_from_custom_room_creation(
        custom_room_id: &'a i32, 
        user_id: &'a i32,
    ) -> Self {
        CustomRoomSlotForm {
            custom_room_id,
            team: &0,
            team_position: &0,
            user_id,
            current_archetype: &Archetypes::Leader,
        }
    }
}