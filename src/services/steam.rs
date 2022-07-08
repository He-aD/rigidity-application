use actix_web::{http, http::uri::Builder, client::Client};
use crate::services::make_path_and_query;
use std::collections::HashMap;
use crate::errors::{AppResult, AppError};
use serde::Deserialize;
use serde_json;

const STEAM_DOMAIN: &str = "partner.steam-api.com";

#[derive(Debug, Deserialize)]
pub struct SteamAuthData {
    pub app_id: u64,
    pub auth_ticket: String
}

#[derive(Deserialize, Debug)]
struct ResponseBase<T> {
    pub response: T
}

#[derive(Deserialize, Debug)]
struct Response<T> {
    pub params: T
}

#[derive(Deserialize, Debug)]
struct AuthenticateUserTicketResponse {
    pub result: String,
    #[serde(rename = "steamid")]
    pub steam_id: String,
    #[serde(rename = "ownersteamid")]
    pub owner_steam_id: String,
    #[serde(rename = "vacbanned")]
    pub vac_banned: bool,
    #[serde(rename = "publisherbanned")] 
    pub publisher_banned: bool,
}

#[derive(Deserialize, Debug)]
struct ErrorResponse {
    #[serde(rename = "errorcode")]
    error_code: i32,
    #[serde(rename = "errordesc")]
    message: String,
}

#[derive(Deserialize, Debug)]
struct Error {
    error: ErrorResponse
}

pub async fn authenticate_user_ticket(data: &SteamAuthData) -> AppResult<String> {
    let mut params = HashMap::new();
    params.insert("key", std::env::var("STEAM_SECRET_ACCESS_KEY").unwrap_or_default());
    params.insert("appid", data.app_id.to_string());
    params.insert("ticket", data.auth_ticket.to_string());

    let uri = Builder::new()
        .scheme("https")
        .authority(STEAM_DOMAIN)
        .path_and_query(make_path_and_query(
            "/ISteamUserAuth/AuthenticateUserTicket/v1", &params))
        .build()
        .unwrap();
    
    let client = Client::default();
    let mut result = client.get(uri)
        .header(http::header::CONTENT_TYPE, "application/x-www-form-urlencoded")
        .send()
        .await?;
    let body = result.body().await?;
    match serde_json::from_slice::<ResponseBase<Response<AuthenticateUserTicketResponse>>>(&body) {
        Ok(steam_response) => {
            if steam_response.response.params.result == "OK" && !steam_response.response.params.vac_banned &&
            ! steam_response.response.params.publisher_banned {
                Ok(steam_response.response.params.steam_id)
            } else {
                Err(AppError::Unauthorized)
            }
        },
        Err(_) => {
            let steam_error = serde_json::from_slice::<ResponseBase<Error>>(&body)?;
            Err(AppError::BadRequest(steam_error.response.error.message))
        }
    }
}