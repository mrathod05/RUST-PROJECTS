pub mod error;
pub mod response;

use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Debug, Serialize)]
pub enum ApiError {
    BadRequest(String),
    Conflict(String),
    Notfound(String),
    InternalServerError(String),
}

#[derive(Serialize, Deserialize)]
pub struct TypeResponse<T> {
    pub code: u16,
    pub success: bool,
    pub message: String,
    pub data: Option<T>,
}
