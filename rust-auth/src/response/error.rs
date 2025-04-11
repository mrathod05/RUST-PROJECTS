use rocket::{http::Status, response::Responder, Response};
use serde_json::json;
use sqlx::{error::ErrorKind, Error as SqlxError};
use std::fmt;

use super::{ApiError, TypeResponse};

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ApiError::BadRequest(msg) => write!(f, "{msg}"),
            ApiError::Conflict(msg) => write!(f, "{msg}"),
            ApiError::Notfound(msg) => write!(f, "{msg}"),
            ApiError::InternalServerError(msg) => write!(f, "{msg}"),
        }
    }
}

impl From<SqlxError> for ApiError {
    fn from(err: SqlxError) -> Self {
        eprintln!("Database error: {}", err);
        match err {
            SqlxError::Database(db_err) => match db_err.kind() {
                ErrorKind::UniqueViolation => {
                    ApiError::BadRequest("This email is already in use".to_string())
                }
                _ => ApiError::InternalServerError("Something went wrong".to_string()),
            },
            _ => ApiError::InternalServerError("Something went wrong".to_string()),
        }
    }
}

impl<'r> Responder<'r, 'static> for ApiError {
    fn respond_to(self, _: &'r rocket::Request<'_>) -> rocket::response::Result<'static> {
        let (status, message) = match self {
            ApiError::BadRequest(msg) => (Status::BadRequest, msg),
            ApiError::Conflict(msg) => (Status::Conflict, msg),
            ApiError::Notfound(msg) => (Status::BadRequest, msg),
            ApiError::InternalServerError(msg) => (Status::BadRequest, msg),
        };

        let error_response = json!(TypeResponse::<()> {
            code: status.code,
            success: false,
            message: message.to_string(),
            data: None,
        });

        let body = error_response.to_string();

        Response::build()
            .status(status)
            .header(rocket::http::ContentType::JSON)
            .sized_body(body.len(), std::io::Cursor::new(body.into_bytes()))
            .ok()
    }
}
