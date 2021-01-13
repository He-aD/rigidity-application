use actix_identity::Identity;
use actix_web::{web, error::BlockingError,HttpResponse};
use serde::Deserialize;
use crate::errors::{AppResult, AppError};
use crate::models::user;
use crate::Pool;
use crate::services::email::EmailService;
use argon2::Config;
use rand::Rng;
use chrono::{Utc, NaiveDateTime};
use crate::app_conf::SECRET_KEY;

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
    match web::block(move || 
        t_login(auth_data, pool)).await {
        Ok(user) => {
            id.remember(user.email.clone());
            Ok(HttpResponse::Ok().json(user))
        }
        Err(err) => match err {
            BlockingError::Error(service_error) => Err(service_error),
            BlockingError::Canceled => Err(AppError::InternalServerError(err.to_string())),
        }
    }
}

fn t_login(
    auth_data: web::Json<AuthData>,
    pool: web::Data<Pool>
) -> AppResult<user::User> {
    let datas = auth_data.into_inner();
    let email = datas.email.clone();
    match user::get(&email, &pool.get().unwrap()) {
        Ok(user) => {
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

#[derive(Debug, Deserialize)]
pub struct AskPassData {
    pub email: String
}

pub async fn ask_password_reset(
    data: web::Json<AskPassData>,
    pool: web::Data<Pool>
) -> AppResult<HttpResponse> {
    match web::block(move || 
        t_ask_password_reset(data, pool)).await {
        Ok(_) => {
            Ok(HttpResponse::Ok().finish())
        }
        Err(err) => match err {
            BlockingError::Error(error) => Err(error),
            BlockingError::Canceled => Err(AppError::InternalServerError(err.to_string())),
        }
    }    
}

fn t_ask_password_reset(
    data: web::Json<AskPassData>,
    pool: web::Data<Pool>
) -> AppResult<()> {
    let hash = new_reset_password_hash()?;
    let result  = user::update_reset_password_hash(
        &data.email, 
        &hash,
        &pool.get().unwrap()
    );

    match result {
        Ok(expire_time) => {
            let url = format!("http://localhost:3000/reset_password.html?id={}", hash);
            let expire_time = NaiveDateTime::from_timestamp(expire_time, 0)
                .format("%c");
            let link = format!("<h1>Hello !</h1><br/><p>Here's your link: {}.</p><p>Your link we'll expire at {} (UTC time)</p>", url, expire_time);
    
            let mut email_service = EmailService::new(
                &data.email,
                String::from("Rigidity password reset"),
                link
            );
            email_service.force_in_debug = true;
            match email_service.send() {
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

fn new_reset_password_hash() -> AppResult<String> {
    let rng = rand::thread_rng().gen::<i64>().to_string();
    hash(&rng)
}

fn hash(to_hash: &str) -> AppResult<String> {
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

#[derive(Debug, Deserialize)]
pub struct ResetPassData {
    pub hash: String,
    pub new_password: String
}

pub async fn reset_password(
    data: web::Json<ResetPassData>,
    pool: web::Data<Pool>
) -> AppResult<HttpResponse> {
    match web::block(move || 
        t_reset_password(data, pool)).await {
        Ok(_) => {
            Ok(HttpResponse::Ok().finish())
        }
        Err(err) => match err {
            BlockingError::Error(error) => Err(error),
            BlockingError::Canceled => Err(AppError::InternalServerError(err.to_string())),
        }
    }    
}

fn t_reset_password(
    data: web::Json<ResetPassData>,
    pool: web::Data<Pool>
) -> AppResult<()> {
    let conn = &pool.get().unwrap();
    let expired_error = Err(AppError::BadRequest(String::from("The link you used has expired. Make a new request.")));

    if let Ok(user) = user::get_by_reset_password_hash(&data.hash, conn) {
        if let Some(expire_date) = user.password_hash_expire_at {
            let now = NaiveDateTime::from_timestamp(Utc::now().timestamp(), 0);
            if expire_date >= now {
                match hash(&data.new_password) {
                    Ok(new_hash) => user::update_password(&data, &new_hash, conn)?,
                    Err(err) => return Err(AppError::InternalServerError(err.to_string()))
                }
            } else {
                if let Err(err) = user::cancel_reset_password_hash(&data.hash, conn) {
                    return Err(AppError::InternalServerError(err.to_string()));
                }
                println!("ex3");
                return expired_error;
            }
        } else {
            println!("ex2");
            return expired_error;
        }
    } else {
        println!("ex1");
        return expired_error;
    }

    Ok(())
}