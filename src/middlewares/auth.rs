use axum::{
    extract::Request, 
    middleware::Next, 
    response::IntoResponse
};
use axum_extra::extract::cookie::CookieJar;
use crate::{
    error::AppError,
    services::auth::get_user_by_session,
    db::get_pool
};

pub async fn auth_guard(
    jar: CookieJar,
    mut req: Request,
    next: Next
) -> impl IntoResponse {
    let get_session_result = jar.get("session");
    match get_session_result {
        Some(session_id) => {
            let user_result = get_user_by_session(
                session_id.value().to_string(), 
                &get_pool().await
            ).await;
            match user_result{
                Ok(user) => {
                    req.extensions_mut().insert(user);
                    return next.run(req).await;
                },
                Err(e) => {
                    return if e == AppError::NotFoundUser {
                        AppError::Unauthorized.into_response()
                    } else {
                        e.into_response()
                    }
                }
            }
        }
        None => return AppError::Unauthorized.into_response()
    }
}