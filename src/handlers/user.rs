use axum::{
    http::{
        HeaderMap,
        HeaderValue, 
        StatusCode
    }, response::IntoResponse, Extension, Json
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
        Ok(user) => {
            let session_result = services::user::create_session(
                &user.username, 
                user.id, 
                &pool
            ).await;
            match session_result {
                Ok(session) => {
                    let mut header = HeaderMap::new();
                    header.insert(
                        axum::http::header::SET_COOKIE,
                        HeaderValue::from_str(
                            &services::user::build_cookie(session)
                        ).unwrap()
                    );
                    return (StatusCode::CREATED, header, Json(user)).into_response()
                }
                Err(e) => return e.into_response()
            }
        },
        Err(e) => return e.into_response()
    }
}

pub async fn login(
    Json(login_dto): Json<modules::user::LoginDto>
) -> impl IntoResponse {
    if let Err(err) = login_dto.validate() {
        return error::AppError::ValidationError(err.to_string()).into_response();
    }
    let pool = get_pool().await;
    let login_result = services::user::login(login_dto, &pool).await;
    match login_result {
        Ok(user) => {
            let session_result = services::user::create_session(
                &user.username,
                user.id,
                &pool
            ).await;
            match session_result {
                Ok(session) => {
                    let mut header = HeaderMap::new();
                    header.insert(
                        axum::http::header::SET_COOKIE,
                        HeaderValue::from_str(
                            &services::user::build_cookie(session)
                        ).unwrap()
                    );
                    return (StatusCode::OK, header, Json(user)).into_response();
                },
                Err(e) => return e.into_response()
            }
        },
        Err(e) => return e.into_response()
    }
}

pub async fn refresh() {}

pub async fn logout() {}

pub async fn update_information() {}

pub async fn update_password() {}

pub async fn delete() {}