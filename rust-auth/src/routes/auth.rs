use crate::db::{SigninRequestDto, SigninResponseDto, SignupRequestDto};
use crate::response::{ApiError, TypeResponse};
use crate::services::auth::AuthServices;

use rocket::serde::json::Json;
use rocket::State;
use sqlx::PgPool;

#[post("/sign-up", format = "json", data = "<sign_up_dto>")]
pub async fn sing_up(
    pool: &State<PgPool>,
    sign_up_dto: Json<SignupRequestDto>,
) -> Result<Json<TypeResponse<()>>, ApiError> {
    AuthServices::sing_up(pool.inner(), sign_up_dto).await
}

#[post("/sign-in", format = "json", data = "<sign_in_dto>")]
pub async fn sing_in(
    pool: &State<PgPool>,
    sign_in_dto: Json<SigninRequestDto>,
) -> Result<Json<TypeResponse<SigninResponseDto>>, ApiError> {
    AuthServices::sing_in(pool.inner(), sign_in_dto).await
}
