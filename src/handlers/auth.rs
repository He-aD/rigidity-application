use actix_identity::Identity;
use actix_web::{web, HttpResponse};
use actix_web::http::StatusCode;
use serde::Deserialize;
use crate::errors::{AppResult, AppError};
use crate::models::user::{self};
use crate::Pool;
use crate::services::{email::EmailService, steam::SteamAuthData};
use chrono::NaiveDateTime;
use crate::app_conf::get_base_url;
use crate::services::{steam, auth as auth_service};

#[derive(Debug, Deserialize)]
pub struct AuthData {
    pub email: String,
    pub password: String
}

pub async fn login(
    auth_data: web::Json<AuthData>,
    id: Identity,
    pool: web::Data<Pool>
) -> AppResult<HttpResponse> {    
    let user = web::block(move || t_login(auth_data, pool)).await?;

    id.remember(user.id.to_string());

    Ok(HttpResponse::Ok().json(user))
}

fn t_login(
    auth_data: web::Json<AuthData>,
    pool: web::Data<Pool>
) -> AppResult<user::User> {
    let datas = auth_data.into_inner();
    let email = datas.email.clone();
    match user::get_by_email(&email, &pool.get().unwrap()) {
        Ok(user) => {
            if !user.can_login() {
                return Err(AppError::Forbidden);
            }
            
            if user.is_password_ok(&datas.password)? {
                return Ok(user);
            }
        }
        Err(_err) => {
            return Err(AppError::BadRequest(String::from("Unknown email.")));
        }
    }

    Err(AppError::BadRequest(String::from("Incorrect password.")))
}

pub async fn login_steam(
    auth_data: web::Json<SteamAuthData>,
    id: Identity,
    pool: web::Data<Pool>
) -> AppResult<HttpResponse> {
    let steam_id = steam::authenticate_user_ticket(&auth_data).await?;

    match web::block(move || 
        user::get_by_steam_id(&steam_id.to_string(), &pool.get().unwrap())).await {
        Ok(user) => {
            steam::check_app_ownership(&auth_data.app_id, &steam_id).await?;
            if user.can_login() {
                id.remember(user.id.to_string());
                return Ok(HttpResponse::Ok().json(user));
            }
            
            Ok(HttpResponse::Forbidden().json(user))
        }
        Err(_err) => {
            Ok(HttpResponse::Ok().status(StatusCode::SEE_OTHER).finish())
        }
    }
}

pub async fn logout(
    id: Identity,
) -> AppResult<HttpResponse> {    
    id.forget();
    Ok(HttpResponse::Ok().finish())
}

#[derive(Debug, Deserialize)]
pub struct AskPassData {
    pub email: String
}

pub async fn ask_password_reset(
    data: web::Json<AskPassData>,
    pool: web::Data<Pool>
) -> AppResult<HttpResponse> {
    match t_ask_password_reset(data, pool).await {
        Ok(_) => Ok(HttpResponse::Ok().finish()),
        Err(err) => Err(err)
    }    
}

async fn t_ask_password_reset(
    data: web::Json<AskPassData>,
    pool: web::Data<Pool>
) -> AppResult<()> {
    let hash = auth_service::new_reset_password_hash()?;
    let result  = user::update_reset_password_hash(
        &data.email, 
        &hash,
        &pool.get().unwrap()
    );

    match result {
        Ok(expire_time) => {
            let url = format!("{}/static/reset_password.html?id={}", get_base_url(), hash);
            let expire_time = NaiveDateTime::from_timestamp(expire_time, 0)
                .format("%c");
            let link = format!("<h1>Hello !</h1><br/><p>Here's your link: {}.</p><p>Your link we'll expire at {} (UTC time)</p>", url, expire_time);
    
            let email_service = EmailService::new(
                &data.email,
                String::from("Rigidity password reset"),
                link
            );
            
            match email_service.send().await {
                Ok(_response) => {
                    Ok(())
                }
                Err(err) => {
                    Err(AppError::ServiceUnavailable(format!("Email service unavailable. Message: {}", err)))
                }
            }
        }
        Err(err) => {
            return Err(AppError::InternalServerError(format!("Database error: {}", err.to_string())));
        }
    } 
}

#[derive(Debug, Deserialize)]
pub struct ResetPassData {
    pub hash: String,
    pub new_password: String
}

pub async fn reset_password(
    data: web::Json<ResetPassData>,
    pool: web::Data<Pool>
) -> AppResult<HttpResponse> {
    web::block(move || t_reset_password(data, pool)).await?;

    Ok(HttpResponse::TemporaryRedirect()
        .status(StatusCode::SEE_OTHER)
        .set_header(
            "Location", 
            "/login.html")
        .finish())
}

fn t_reset_password(
    data: web::Json<ResetPassData>,
    pool: web::Data<Pool>
) -> AppResult<()> {
    let conn = &pool.get().unwrap();

    auth_service::check_reset_password_hash(&data.hash, conn)?;
    match auth_service::hash_password(&data.new_password) {
        Ok(new_hash) => user::update_password(&data, &new_hash, conn)?,
        Err(err) => return Err(AppError::InternalServerError(err.to_string()))
    }

    Ok(())
}

pub async fn refresh_cookie(
    id: Identity,
) -> AppResult<HttpResponse> {
    let user_id = id.identity().unwrap();
    id.forget();
    id.remember(user_id);
    Ok(HttpResponse::Ok().finish())
}

#[derive(Deserialize)]
pub struct EmailConfirmationData {
    pub hash: String
}

pub async fn email_confirmation(
    data: web::Json<EmailConfirmationData>,
    pool: web::Data<Pool>
) -> AppResult<HttpResponse> {
    web::block(move ||
        auth_service::email_confirmation(&data.hash, &pool.get().unwrap())).await?; 

    Ok(HttpResponse::Ok().finish())
}

#[derive(Deserialize)]
pub struct UpdateEmailConfirmationData {
    pub email: String,
    pub auth: steam::SteamAuthData
}

pub async fn update_email_confirmation(
    data: web::Json<UpdateEmailConfirmationData>,
    pool: web::Data<Pool>
) -> AppResult<HttpResponse> {
    let steam_id = auth_service::steam_authenticate_and_ownership_check(&data.auth).await?;

    auth_service::update_email_confirmation(
        data.email.clone(), steam_id, pool).await?;
    
    Ok(HttpResponse::Ok().finish())
}