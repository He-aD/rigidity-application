use diesel_derive_enum::DbEnum;
use serde::{Deserialize, Serialize};
use std::fmt::{Formatter, Result, Display};

#[derive(Eq, Hash, Deserialize, PartialEq, Serialize, Debug, DbEnum)]
#[PgType = "enum_archetypes"]
#[DieselType = "Enum_archetypes"]
pub enum Archetypes {
    #[db_rename = "leader"]
    Leader,
    #[db_rename = "spiker"]
    Spiker,
    #[db_rename = "healer"]
    Healer,
    #[db_rename = "assassin"]
    Assassin,
}

impl Archetypes {
    pub fn from_u32(value: u32) -> Option<Archetypes> {
        match value {
            0 => Some(Archetypes::Leader),
            1 => Some(Archetypes::Spiker),
            2 => Some(Archetypes::Healer),
            3 => Some(Archetypes::Assassin),
            _ => None,
        }
    }

    pub fn to_u32(&self) -> u32 {
        match self {
            Archetypes::Leader => 0,
            Archetypes::Spiker => 1,
            Archetypes::Healer => 2,
            Archetypes::Assassin => 3,
        }
    }
}

impl Display for Archetypes {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Eq, Hash, Deserialize, PartialEq, Serialize, Debug, DbEnum)]
#[PgType = "enum_game_modes"]
#[DieselType = "Enum_game_modes"]
pub enum GameModes {
    #[db_rename = "deathmatch"]
    Deathmatch,
    #[db_rename = "king_of_the_hill"]
    KingOfTheHill
}

impl Display for GameModes {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Eq, Hash, Deserialize, PartialEq, Serialize, Debug, DbEnum)]
#[PgType = "enum_maps"]
#[DieselType = "Enum_maps"]
pub enum Maps {
    #[db_rename = "heaven"]
    Heaven,
    #[db_rename = "ascent"]
    Ascent, 
    #[db_rename = "inferno"]
    Inferno,
    #[db_rename = "colosseum"]
    Colosseum,
    #[db_rename = "play_ground"]
    PlayGround,
}

impl Display for Maps {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{:?}", self)
    }
}