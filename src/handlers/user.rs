use axum::{
    http::StatusCode,
    response::IntoResponse,
    Json
};
use validator::Validate;
use crate::{
    modules,
    error,
    services,
    db::get_pool
};


pub async fn register(
    Json(create_dto): Json<modules::user::CreateDto>
) -> impl IntoResponse {
    if let Err(err) = create_dto.validate() {
        return error::AppError::ValidationError(err.to_string()).into_response();
    }
    let pool = get_pool().await;
    let create_result = services::user::create(create_dto, &pool).await;
    match create_result {
        Ok(user) => return (StatusCode::CREATED, Json(user)).into_response(),
        Err(e) => return e.into_response()
    }
}

pub async fn login() {}

pub async fn refresh() {}

pub async fn logout() {}

pub async fn update_information() {}

pub async fn update_password() {}

pub async fn delete() {}