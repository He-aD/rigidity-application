use actix_web::{error::ResponseError, HttpResponse};
use derive_more::Display;
use diesel::result::{DatabaseErrorKind, Error as DBError};
use std::convert::From;

#[derive(Debug, Display)]
pub enum AppError {
    #[display(fmt = "Internal Server Error")]
    InternalServerError,
    
    #[display(fmt = "BadRequest: {}", _0)]
    BadRequest(String),

    #[display(fmt = "Unauthorized")]
    Unauthorized,
}

// impl ResponseError trait allows to convert our errors into http responses with appropriate data
impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::InternalServerError => HttpResponse::InternalServerError()
                .json("Internal Server Error, Please try later"),
            AppError::BadRequest(ref message) => {
                HttpResponse::BadRequest().json(message)
            }
            AppError::Unauthorized => {
                HttpResponse::Unauthorized().json("Unauthorized")
            }
        }
    }
}

impl From<DBError> for AppError {
    fn from(error: DBError) -> AppError {
        // Right now we just care about UniqueViolation from diesel
        // But this would be helpful to easily map errors as our app grows
        match error {
            DBError::DatabaseError(kind, info) => {
                if let DatabaseErrorKind::UniqueViolation = kind {
                    let message =
                        info.details().unwrap_or_else(|| info.message()).to_string();
                    return AppError::BadRequest(message);
                }
                AppError::InternalServerError
            }
            _ => AppError::InternalServerError,
        }
    }
}