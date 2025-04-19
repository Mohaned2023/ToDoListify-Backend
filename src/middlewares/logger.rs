use axum::{
    extract::Request,
    middleware::Next, 
    response::IntoResponse
};
use tracing::info;

pub async fn log_request(req: Request, next: Next) -> impl IntoResponse {
    let method = req.method().clone();
    let path = req.uri().path().to_string();
    info!("{} '{}'", method, path);
    next.run(req).await
}