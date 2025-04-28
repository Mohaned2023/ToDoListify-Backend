use axum::{
    http::{
        HeaderValue,
        Method
    }, 
    Router
};
use dotenvy::dotenv;
use tracing::info;
use tower_http::cors::CorsLayer;

mod routes;
mod handlers;
mod middlewares;
mod modules;
mod error;
mod db;
mod services;

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt::init();
    let frontend_url = std::env::var("TODOLISTIFY_APP_FRONTEND_URL")
        .expect(">>> TODOLISTIFY_APP_FRONTEND_URL NOT found!");
    let cors_layer = CorsLayer::new()
        .allow_origin(frontend_url.parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE]);
    let app = Router::new()
        .nest("/api/v1", routes::main())
        .layer(axum::middleware::from_fn(middlewares::logger::log_request))
        .layer(cors_layer);
    let port = std::env::var("TODOLISTIFY_APP_PORT")
        .expect(">>> TODOLISTIFY_APP_PORT NOT found!");
    let listener = tokio::net::TcpListener::bind(
        format!("127.0.0.1:{}", port)
    )
        .await
        .expect(">>> Can NOT create the listener!");
    info!("App running on: {}", listener.local_addr().unwrap());
    axum::serve(listener, app)
        .await
        .expect(">>> Axum can NOT serve us!");
}
