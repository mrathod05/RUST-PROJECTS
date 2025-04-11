use sqlx::PgPool;

use crate::{
    db::{user::PublicUser, SignupRequestDto},
    response::ApiError,
};

use super::User;

impl User {
    pub async fn create(pool: &PgPool, data: SignupRequestDto) -> Result<PublicUser, ApiError> {
        sqlx::query_as::<_, PublicUser>(
            "INSERT INTO users (username, email, password) 
             VALUES ($1, $2, $3) 
             RETURNING id, username, email,password, created_at",
        )
        .bind(&data.username)
        .bind(&data.email)
        .bind(&data.password)
        .fetch_one(pool)
        .await
        .map_err(ApiError::from)
    }

    pub async fn _get_all(pool: &PgPool) -> Result<Vec<PublicUser>, ApiError> {
        sqlx::query_as::<_, PublicUser>("Select * from users")
            .fetch_all(pool)
            .await
            .map_err(ApiError::from)
    }

    pub async fn get_one(pool: &PgPool, email: &String) -> Result<Option<User>, ApiError> {
        sqlx::query_as("SELECT id, username, email, password,created_at FROM users WHERE email=$1")
            .bind(email)
            .fetch_optional(pool)
            .await
            .map_err(ApiError::from)
    }
}
