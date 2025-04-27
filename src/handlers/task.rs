use axum::{
    extract::Path, 
    http::StatusCode, 
    response::IntoResponse, 
    Extension, 
    Json
};
use validator::Validate;

use crate::{
    error, 
    modules,
    services,
    db::get_pool
};


pub async fn get_all(
    Extension(user): Extension<modules::user::User>
) -> impl IntoResponse {
    let get_result = services::task::get_all(
        user.id, 
        &get_pool().await
    ).await;
    match get_result {
        Ok(tasks) => return (StatusCode::OK, Json(tasks)).into_response(),
        Err(e) => return e.into_response()
    }
}

pub async fn create(
    Extension(user): Extension<modules::user::User>,
    Json(create_dto): Json<modules::task::CreateDto>
) -> impl IntoResponse {
    if let Err(err) = create_dto.validate() {
        return error::AppError::ValidationError(err.to_string()).into_response();
    }
    let create_result = services::task::create(
        create_dto, 
        user.id, 
        &get_pool().await
    ).await;
    match create_result {
        Ok(task) => return (StatusCode::CREATED, Json(task)).into_response(),
        Err(e) => return e.into_response()
    };
}

pub async fn update(
    Path(id): Path<i32>,
    Extension(user): Extension<modules::user::User>,
    Json(update_dto): Json<modules::task::UpdateDto>
) -> impl IntoResponse {
    if let Err(e) = update_dto.validate() {
        return error::AppError::ValidationError(e.to_string()).into_response();
    }
    let updated_result = services::task::udpate(
        update_dto, 
        id, 
        user.id, 
        &get_pool().await
    ).await;
    match updated_result {
        Ok(task) => return (StatusCode::OK, Json(task)).into_response(),
        Err(e) => return e.into_response()
    }
}

pub async fn delete(
    Path(id): Path<i32>,
    Extension(user): Extension<modules::user::User>
) -> impl IntoResponse {
    let deleted_result = services::task::delete(
        id, 
        user.id, 
        &get_pool().await
    ).await;
    match deleted_result {
        Ok(_) => return (StatusCode::OK).into_response(),
        Err(e) => return e.into_response()
    }
}