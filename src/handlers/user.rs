use serde::{Deserialize};
use crate::chrono::{DateTime, Utc};
use actix_web::{HttpResponse, error::{BlockingError}, web};
use crate::Pool;
use crate::{errors::{AppResult, AppError}};
use crate::models::user::{create as create_user};
use crate::models::forms::user::UserForm;
use crate::services::{steam, auth as auth_service};

#[derive(Deserialize)]
pub struct CreateUserData {
    pub email: String,
    pub nickname: String,
    pub first_name: String,
    pub last_name: String,
    pub birth_date: DateTime<Utc>,
    pub auth: steam::SteamAuthData,
}

pub async fn create(
    create_data: web::Json<CreateUserData>,
    pool: web::Data<Pool>
) -> AppResult<HttpResponse> {    
    let steam_id = auth_service::steam_authenticate_and_ownership_check(&create_data.auth).await?;
    let data = create_data.into_inner();
    let email_confirmation_hash = auth_service::new_reset_password_hash()?;

    match web::block(move || 
        create_user(
            UserForm::new_from_data(&data, &steam_id.to_string()), 
            &email_confirmation_hash,&pool.get().unwrap())).await {
        Ok((user, expire_timestamp)) => {
            match &user.reset_password_hash {
                Some(hash) => {
                    let _r = auth_service::send_confirmation_email(&user.email, expire_timestamp, &hash);
                    Ok(HttpResponse::Ok().json(user))
                }
                None => return Err(AppError::InternalServerError(
                    format!("Reset password hash was not set up properly.")))
            }
        }
        Err(err) => match err {
            BlockingError::Error(model_err) => Err(AppError::BadRequest(model_err.to_string())),
            BlockingError::Canceled => Err(AppError::InternalServerError(err.to_string())),
        }
    }
}
