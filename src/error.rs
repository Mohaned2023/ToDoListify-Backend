use axum::{
    http::StatusCode,
    response::{
        IntoResponse, 
        Response
    }, 
    Json
};
use serde_json::json;


pub enum AppError {
    ValidationError(String),
    UserFound,
    InternalServer
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::ValidationError(err) => (StatusCode::BAD_REQUEST, err),
            AppError::UserFound => (StatusCode::FOUND, "User already registered!".to_string()),
            AppError::InternalServer => (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error!".to_string())
        };
        (
            status,
            Json(json!({
                "error": message,
                "status": status.as_u16()
            }))
        ).into_response()
    }
}