use rocket::{
    http::Status,
    serde::{json::Json, Serialize},
};

use super::TypeResponse;

impl<T: Serialize> TypeResponse<T> {
    pub fn send(status: Status, data: Option<T>, message: &str) -> Json<Self> {
        Json(Self {
            code: status.code,
            data,
            message: message.to_string(),
            success: true,
        })
    }
}
