use diesel_derive_enum::DbEnum;
use serde::{Deserialize, Serialize};

#[derive(Eq, Hash, Deserialize, PartialEq, Serialize, Debug, DbEnum)]
#[PgType = "enum_archetypes"]
#[DieselType = "Enum_archetypes"]
pub enum Archetypes {
    #[db_rename = "leader"]
    Leader,
    #[db_rename = "healer"]
    Healer,
    #[db_rename = "spiker"]
    Spiker
}

#[derive(Eq, Hash, Deserialize, PartialEq, Serialize, Debug, DbEnum)]
#[PgType = "enum_game_modes"]
#[DieselType = "Enum_game_modes"]
pub enum GameModes {
    #[db_rename = "deathmatch"]
    Deathmatch,
}

#[derive(Eq, Hash, Deserialize, PartialEq, Serialize, Debug, DbEnum)]
#[PgType = "enum_maps"]
#[DieselType = "Enum_maps"]
pub enum Maps {
    #[db_rename = "heaven"]
    Heaven,
}