use std::env;

use crate::db::user::User;
use crate::db::{SigninRequestDto, SigninResponseDto, SignupRequestDto};
use crate::response::{ApiError, TypeResponse};
use crate::utils::{generate_jwt, hash_password, verify_password, JwtUserSubject};

use rocket::{http::Status, serde::json::Json};
use sqlx::PgPool;

pub struct AuthServices;

impl AuthServices {
    pub async fn sing_up(
        pool: &PgPool,
        data: Json<SignupRequestDto>,
    ) -> Result<Json<TypeResponse<()>>, ApiError> {
        let result = hash_password(&data.password);

        let hash_password = match result {
            Ok(password) => password,
            Err(_) => {
                return Err(ApiError::InternalServerError(
                    "Something went wrong".to_string(),
                ))
            }
        };

        let _ = User::create(
            pool,
            SignupRequestDto {
                username: data.username.clone(),
                email: data.email.clone(),
                password: hash_password,
            },
        )
        .await?;

        Ok(TypeResponse::send(
            Status::Ok,
            Some(()),
            "Signup Successful",
        ))
    }

    pub async fn sing_in(
        pool: &PgPool,
        data: Json<SigninRequestDto>,
    ) -> Result<Json<TypeResponse<SigninResponseDto>>, ApiError> {
        let mut user = User::get_one(pool, &data.email)
            .await?
            .ok_or_else(|| ApiError::BadRequest("Invalid email or password".to_string()))?;

        let user_id = user
            .id
            .ok_or_else(|| ApiError::InternalServerError("Something went wrong".to_string()))?;

        if !verify_password(data.password.as_str(), &user.password) {
            return Err(ApiError::BadRequest(
                "Invalid email or password".to_string(),
            ));
        }

        user.password = "".to_string();

        let secret = env::var("JWT_SECRET").expect("Must have Secrete");

        let token = generate_jwt(
            JwtUserSubject {
                id: user_id,
                username: user.username.clone(),
                email: user.email.clone(),
            },
            secret.as_str(),
        )
        .map_err(|_| ApiError::InternalServerError("Failed to generate JWT".to_string()))?;

        let response_body = SigninResponseDto {
            id: user.id.unwrap(),
            username: user.username,
            email: user.email,
            token,
        };

        Ok(TypeResponse::send(
            Status::Ok,
            Some(response_body),
            "SignIn Successful",
        ))
    }
}
