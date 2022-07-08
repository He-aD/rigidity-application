use serde::{Deserialize};
use crate::chrono::{DateTime, Utc};
use actix_web::{HttpResponse, error::{BlockingError}, web};
use crate::Pool;
use actix_identity::Identity;
use crate::{errors::{AppResult, AppError}};
use crate::models::user::{create as create_user};
use crate::models::forms::user::UserForm;
use crate::services::steam;

#[derive(Debug, Deserialize)]
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
    id: Identity,
    pool: web::Data<Pool>
) -> AppResult<HttpResponse> {    
    let steam_id = steam::authenticate_user_ticket(&create_data.auth).await?;

    match web::block(move || 
        create_user(
            UserForm::new_from_data(&create_data.into_inner(), &steam_id), 
            &pool.get().unwrap())).await {
        Ok(user) => {
            id.remember(user.id.to_string());
            Ok(HttpResponse::Ok().json(user))
        }
        Err(err) => match err {
            BlockingError::Error(model_err) => Err(AppError::BadRequest(model_err.to_string())),
            BlockingError::Canceled => Err(AppError::InternalServerError(err.to_string())),
        }
    }
}
