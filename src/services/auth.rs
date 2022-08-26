use argon2::Config;
use diesel::PgConnection;
use rand::Rng;
use crate::app_conf::SECRET_KEY;
use crate::errors::{AppResult, AppError};
use crate::app_conf::get_base_url;
use crate::services::email::EmailService;
use chrono::{Utc, NaiveDateTime};
use crate::models::user::{self};
use crate::services::steam;
use actix_web::web;
use crate::Pool;

pub fn new_reset_password_hash() -> AppResult<String> {
    let rng = rand::thread_rng().gen::<i64>().to_string();
    hash_password(&rng)
}

pub fn hash_password(to_hash: &str) -> AppResult<String> {
    let config = Config {
        secret: SECRET_KEY.as_bytes(),
        ..Default::default()
    };
    
    let salt = std::env::var("SALT").unwrap_or_else(|_| "0123".repeat(8));
    argon2::hash_encoded(to_hash.as_bytes(), &salt.as_bytes(), &config)
        .map_err(|err| {
        AppError::InternalServerError(err.to_string())
    })
}

pub fn check_reset_password_hash(hash: &str, conn: &PgConnection) -> AppResult<()> {
    let expired_error = Err(AppError::BadRequest(String::from("The link you used has expired. Make a new request.")));

    let user = user::get_by_reset_password_hash(hash, conn)?;
    if let Some(expire_date) = user.password_hash_expire_at {
        let now = NaiveDateTime::from_timestamp(Utc::now().timestamp(), 0);
        if expire_date >= now {
            Ok(())
        } else {
            if let Err(err) = user::cancel_reset_password_hash(&hash, conn) {
                return Err(AppError::BadRequest(err.to_string()));
            }
            return expired_error;
        }
    } else {
        return expired_error;
    }
}

pub async fn send_confirmation_email(email: &str, expire_timestamp: i64, hash: &str) -> AppResult<()> {
    let url = format!("{}/static/email_confirmation.html?id={}", get_base_url(), hash);
    let expire_time = NaiveDateTime::from_timestamp(
        expire_timestamp, 0).format("%c");
    let studio_logo_url = format!("{}/static/assets/images/logo_studio.png", 
        get_base_url());
    let link = format!("<p>Hello, </p><p>Welcome to rigidity!</p><p>Please click on the following link to confirm you email address: <a href='{}'>confirm link</a></p><p>Your link we'll expire at {} (UTC time)</p></br></br><img src='{}'>", url, expire_time, studio_logo_url);

    let email_service = EmailService::new(
        email,
        String::from("Rigidity email confirmation"),
        link
    );
    
    email_service.send().await?;
    Ok(())
}

pub fn email_confirmation(hash: &str, conn: &PgConnection) -> AppResult<()> {
    check_reset_password_hash(hash, conn)?;
    user::confirm_email(hash, conn)?;

    Ok(())
}

pub async fn update_email_confirmation(
    email: String, steam_id: u64, pool: web::Data<Pool>) -> AppResult<()> {
    let c_email = email.clone();
    let (expire_time_stamp, email_confirmation_hash) = web::block(move || 
        t_update_email_confirmation(email, steam_id, pool)).await?;
    send_confirmation_email(&c_email, expire_time_stamp, &email_confirmation_hash).await?;
    
    Ok(())
}

pub fn t_update_email_confirmation(
    email: String, steam_id: u64, pool: web::Data<Pool>) -> AppResult<(i64, String)> {
    let conn = &pool.get().unwrap(); 
    let user = user::get_by_steam_id(&steam_id.to_string(), conn)?;

    if user.email_confirmation_required {
        let email_confirmation_hash = new_reset_password_hash()?;
        let (_user, expire_time_stamp) = user::update_email(
            &email, &steam_id, &email_confirmation_hash, conn)?;
            Ok((expire_time_stamp, email_confirmation_hash))
    } else {
        return Err(AppError::Forbidden);
    }
}

pub async fn steam_authenticate_and_ownership_check(
    data: &steam::SteamAuthData) -> AppResult<u64> {
    let steam_id = steam::authenticate_user_ticket(data).await?;
    steam::check_app_ownership(&data.app_id, &steam_id).await?; 

    Ok(steam_id)
}