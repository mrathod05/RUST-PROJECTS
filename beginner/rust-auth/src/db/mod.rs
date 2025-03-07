pub mod connection;
pub mod user;

use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Debug, Deserialize, Serialize)]

/// **Signup request Data Transfer Object (DTO)**
/// This struct represents the request payload for user signup.
pub struct SignupRequestDto {
    pub username: String,
    pub email: String,
    pub password: String,
}

/// **Signin request Data Transfer Object (DTO)**
/// This struct represents the request payload for user signin.
#[derive(Debug, Deserialize, Serialize)]
pub struct SigninRequestDto {
    pub email: String,
    pub password: String,
}

/// **Signin response Data Transfer Object (DTO)**
/// This struct represents the response payload for user signin.
#[derive(Debug, Clone, Deserialize, Serialize, FromRow)]
pub struct SigninResponseDto {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub token: String,
}
