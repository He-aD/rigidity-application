use actix_web::{error::{ResponseError, PayloadError}, HttpResponse, client::SendRequestError};
use derive_more::Display;
use diesel::result::{DatabaseErrorKind, Error as DBError};
use std::convert::From;
use serde_json;

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

    #[display(fmt = "Forbidden")]
    Forbidden,
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
            }
            AppError::BadRequest(ref message) => {
                HttpResponse::BadRequest().json(message)
            }
            AppError::Unauthorized => {
                HttpResponse::Unauthorized().json("Unauthorized")
            }
            AppError::Forbidden => {
                HttpResponse::Forbidden().json("Forbidden")
            }
        }
    }
}

impl From<DBError> for AppError {
    fn from(error: DBError) -> AppError {
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

impl From<serde_json::Error> for AppError {
    fn from(error: serde_json::Error) -> AppError {
        AppError::InternalServerError(format!("Error while parsing json. {}", error.to_string()))
    }
}

impl From<PayloadError> for AppError {
    fn from(error: PayloadError) -> AppError {
        AppError::InternalServerError(format!("Payload error. {}", error.to_string()))
    }
}

impl From<SendRequestError> for AppError {
    fn from(error: SendRequestError) -> AppError {
        AppError::BadRequest(format!("A request send by the server has failed. {}", error.to_string()))
    }
}
