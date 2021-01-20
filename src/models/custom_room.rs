use crate::{schema::{custom_room_slots, custom_rooms}};
use crate::diesel::prelude::*;
use diesel::{PgConnection};
use serde::{Deserialize, Serialize};
use crate::enums::{Archetypes, GameModes, Maps};
use crate::models::ORMResult;
use std::collections::HashMap;
use std::cmp::Eq;
use crate::handlers::custom_room::{CreateData};
use diesel::result::Error;
use form::{CustomRoomForm, CustomRoomSlotForm};

pub mod form;

#[derive(Eq, Hash, Insertable, Identifiable, Serialize, Deserialize, Queryable, PartialEq)]
pub struct CustomRoom {
    pub id: i32,
    pub label: String,
    pub user_id: i32,
    pub nb_teams: i32,
    pub max_player_per_team: i32,
    pub current_game_mode: GameModes,
    pub current_map: Maps,
}

impl CustomRoom {
    pub fn get_capacity(&self) -> usize {
        let capacity = (self.nb_teams * self.max_player_per_team) as usize;
        capacity
    }
}

#[derive(Insertable, Identifiable, Serialize, Deserialize, Queryable, Associations, PartialEq)]
#[belongs_to(CustomRoom)]
pub struct CustomRoomSlot {
    pub id: i32,
    pub custom_room_id: i32,
    pub team: i32,
    pub team_position: i32,
    pub user_id: i32,
    pub current_archetype: Archetypes,
}

pub fn get(id: &i32, conn: &PgConnection)
-> ORMResult<(CustomRoom, Vec<CustomRoomSlot>)> {
    use crate::schema::custom_rooms::dsl::{id as cr_id, custom_rooms};

    let custom_room = custom_rooms
        .filter(cr_id.eq(id))
        .get_result::<CustomRoom>(conn)?;

    let slots = CustomRoomSlot::belonging_to(&custom_room)
        .load::<CustomRoomSlot>(conn)?;
    
    Ok((custom_room, slots))
}

pub fn get_all(conn: &PgConnection) 
-> ORMResult<HashMap<CustomRoom, Vec<CustomRoomSlot>>> {
    use crate::schema::custom_rooms::dsl::*;

    match custom_rooms.load::<CustomRoom>(conn) {
        Ok(c_rs) => {
            let mut result = HashMap::new();
            for c_r in c_rs {
                match CustomRoomSlot::belonging_to(&c_r)
                    .load::<CustomRoomSlot>(conn) {
                    Ok(slots) => {
                        result.insert(c_r, slots);
                    }
                    Err(err) => return Err(err)
                }
            }
            Ok(result)
        }
        Err(err) => return Err(err)
    }
}

pub fn create(
    user_id: &i32, 
    data: CreateData,
    conn: &PgConnection
) -> ORMResult<(CustomRoom, Vec<CustomRoomSlot>)> {
    use crate::schema::custom_rooms::dsl::{id, custom_rooms};
    use crate::schema::custom_room_slots::dsl::{custom_room_slots};

    conn.transaction::<(CustomRoom, Vec<CustomRoomSlot>), Error, _>(move || {
        diesel::insert_into(custom_rooms)
            .values(CustomRoomForm::new_from_createdata(
                &data, 
                user_id))
            .execute(conn)?;

        let custom_room_id = custom_rooms
            .select(id)
            .order(id.desc())
            .first(conn)?;

        diesel::insert_into(custom_room_slots)
            .values(CustomRoomSlotForm::new_from_custom_room_creation(
                &custom_room_id, 
                user_id))
            .execute(conn)?;

        get(&custom_room_id, conn)       
    })
}

pub fn create_custom_room_slot(
    custom_room_slot_form: &CustomRoomSlotForm,
    conn: &PgConnection
) -> ORMResult<(CustomRoom, Vec<CustomRoomSlot>)> {
    use crate::schema::custom_room_slots::dsl::{custom_room_slots};

    diesel::insert_into(custom_room_slots)
        .values(custom_room_slot_form)
        .execute(conn)?;

    get(&custom_room_slot_form.get_custom_room_id(), conn)  
} 