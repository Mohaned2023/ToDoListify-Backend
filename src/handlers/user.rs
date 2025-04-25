use axum::{
    http::{
        HeaderMap,
        HeaderValue, 
        StatusCode
    }, 
    response::IntoResponse, 
    Extension, 
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
        Ok(user) => {
            let session_result = services::auth::create_session(
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
                            &services::auth::build_cookie(session)
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
            let session_result = services::auth::create_session(
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
                            &services::auth::build_cookie(session)
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

pub async fn refresh(
    Extension(user): Extension<modules::user::User>
) -> impl IntoResponse {
    let create_session_result = services::auth::create_session(
        &user.username,
        user.id, 
        &get_pool().await
    ).await;
    match create_session_result {
        Ok(session) => {
            let mut header = HeaderMap::new();
            header.insert(
                axum::http::header::SET_COOKIE,
                HeaderValue::from_str(
                    &services::auth::build_cookie(session)
                ).unwrap()
            );
            return (StatusCode::OK, header, Json(user)).into_response();
        }
        Err(e) => return e.into_response()
    }
}

pub async fn logout(
    Extension(user): Extension<modules::user::User>
) -> impl IntoResponse { 
    let delete_session_result = services::auth::delete_session(
        user.id,
        &get_pool().await
    ).await;
    match delete_session_result {
        Ok(_) => {
            let mut header = HeaderMap::new();
            header.insert(
                axum::http::header::SET_COOKIE,
                HeaderValue::from_str(
                    &services::auth::build_deleted_cookie()
                ).unwrap()
            );
            return (StatusCode::OK, header).into_response();
        },
        Err(e) => return e.into_response()
    }
}

pub async fn update_information(
    Extension(user): Extension<modules::user::User>,
    Json(update_info_dto): Json<modules::user::UpdateInformationDto>,
) -> impl IntoResponse {
    if let Err(e) = update_info_dto.validate() {
        return error::AppError::ValidationError(e.to_string()).into_response();
    }
    if  update_info_dto.email.is_none() &&
        update_info_dto.name.is_none()  &&
        update_info_dto.username.is_none() {
        return error::AppError::BadRequest.into_response();
    }
    let pool = get_pool().await;
    let updated_result = services::user::update_information(
        update_info_dto, 
        user, 
        &pool
    ).await;
    match updated_result {
        Ok(user) => {
            let session_result = services::auth::create_session(
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
                            &services::auth::build_cookie(session)
                        ).unwrap()
                    );
                    return (StatusCode::CREATED, header, Json(user)).into_response()
                }
                Err(e) => return e.into_response()
            }
        }
        Err(e) => return e.into_response()
    }
}

pub async fn update_password(
    Extension(user): Extension<modules::user::User>,
    Json(update_pass_dto): Json<modules::user::UpdatePasswordDto>
) -> impl IntoResponse {
    let updated_result = services::user::update_password(
        update_pass_dto, 
        user, 
        &get_pool().await
    ).await;
    match updated_result {
        Ok( () ) => return (StatusCode::OK).into_response(),
        Err(e) => return e.into_response()
    }
}

pub async fn delete() {}