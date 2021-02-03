table! {
    use diesel::sql_types::*;
    use crate::enums::*;

    custom_room_slots (id) {
        id -> Int4,
        custom_room_id -> Int4,
        team -> Int4,
        team_position -> Int4,
        user_id -> Int4,
        current_archetype -> Enum_archetypes,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::enums::*;

    custom_rooms (id) {
        id -> Int4,
        label -> Varchar,
        user_id -> Int4,
        nb_teams -> Int4,
        max_player_per_team -> Int4,
        current_game_mode -> Enum_game_modes,
        current_map -> Enum_maps,
        matchmaking_ticket -> Nullable<Uuid>,
    }
}

table! {
    use diesel::sql_types::*;

    users (id) {
        id -> Int4,
        email -> Varchar,
        nickname -> Varchar,
        hash -> Varchar,
        reset_password_hash -> Nullable<Varchar>,
        password_hash_expire_at -> Nullable<Timestamp>,
        created_at -> Timestamp,
    }
}

joinable!(custom_room_slots -> custom_rooms (custom_room_id));
joinable!(custom_room_slots -> users (user_id));
joinable!(custom_rooms -> users (user_id));

allow_tables_to_appear_in_same_query!(
    custom_room_slots,
    custom_rooms,
    users,
);
