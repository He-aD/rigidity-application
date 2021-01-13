use actix_web::{error::ResponseError, HttpResponse};
use derive_more::Display;
use diesel::result::{DatabaseErrorKind, Error as DBError};
use std::convert::From;

pub type AppResult<R> = Result<R, AppError>;

#[derive(Debug, Display)]
pub enum AppError {
    #[display(fmt = "Service Unavailable")]
    ServiceUnavailable(String),

    #[display(fmt = "Internal Server Error")]
    InternalServerError(String),
    
    #[display(fmt = "BadRequest: {}", _0)]
    BadRequest(String),

    #[display(fmt = "Unauthorized")]
    Unauthorized,
}

// impl ResponseError trait allows to convert our errors into http responses with appropriate data
impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::ServiceUnavailable(ref message) => HttpResponse::ServiceUnavailable()
                .json(message),
            AppError::InternalServerError(ref trace) => {
                println!("Internal Server Error trace: {}", trace);
                HttpResponse::InternalServerError()
                .json("Internal Server Error, Please try later")
            },
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
                AppError::InternalServerError(String::from("Database error"))
            }
            _ => AppError::InternalServerError(String::from("Database error")),
        }
    }
}