use rusoto_gamelift::GameLiftClient;
use rusoto_core::credential::{EnvironmentProvider};
use rusoto_core::request::HttpClient;
use rusoto_core::region::Region;
use serde::Deserialize;
use rusoto_core::region;
use std::fmt::{Display, Formatter, Result as FmtResult};

pub async fn get_gamelift_client() -> GameLiftClient {
    let cred = EnvironmentProvider::default();
    let client = HttpClient::new().unwrap();
    GameLiftClient::new_with(client, cred, Region::EuWest1)
}

#[derive(Deserialize, Debug)]
pub enum FlexMatchEvents {
    MatchmakingSearching,
    PotentialMatchCreated,
    AcceptMatch,
    AcceptMatchCompleted,
    MatchmakingSucceeded,
    MatchmakingTimedOut,
    MatchmakingCancelled,
    MatchmakingFailed,
}

impl Display for FlexMatchEvents {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{:?}", self)
    }
}

pub enum GameLiftConfiguration {
    CustomGame
}

#[derive(Deserialize, Debug)]
pub struct FlexMatchData<T> 
{
    pub id: String,
    pub account: String,
    pub region: region::Region,
    pub resources: Vec<String>,
    pub detail: T
}

impl<T> FlexMatchData<T> {
    pub fn get_configuration(&self) -> Result<GameLiftConfiguration, String> {
        let split: Vec<&str> = self.resources[0].split("/").collect();
        match split[1] {
            "CustomGame" => {
                Ok(GameLiftConfiguration::CustomGame)
            },
            _ => {
                Err(String::from("Unknown gamelift configuration."))
            }
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct FlexMatchDetail {
    pub tickets: Vec<FlexMatchTicket>,
    #[serde(rename = "type")]
    pub e_type: FlexMatchEvents
}

#[derive(Deserialize, Debug)]
pub struct FlexMatchPotentialDetail {
    pub tickets: Vec<FlexMatchTicket>,
    #[serde(rename = "type")]
    pub e_type: FlexMatchEvents,
    #[serde(rename = "matchId")]
    pub match_id: String
}

#[derive(Deserialize, Debug)]
pub struct FlexMatchSucceededDetail {
    pub tickets: Vec<FlexMatchTicket>,
    #[serde(rename = "type")]
    pub e_type: FlexMatchEvents,
    #[serde(rename = "matchId")]
    pub match_id: String,
    #[serde(rename = "gameSessionInfo")]
    pub game_session_info: FlexMatchGameSession
}

#[derive(Deserialize, Debug)]
pub struct FlexMatchGameSession {
    #[serde(rename = "ipAddress")]
    pub ip_address: String,
    pub port: i32,
    pub players: Vec<FlexMatchPlayerGameSession>
}

#[derive(Deserialize, Debug)]
pub struct FlexMatchPlayerGameSession {
    #[serde(rename = "playerId")]
    pub player_id: String,
    #[serde(rename = "playerSessionId")]
    pub player_session_id: String
}

#[derive(Deserialize, Debug)]
pub struct FlexMatchTicket {
    pub ticket_id: String
}