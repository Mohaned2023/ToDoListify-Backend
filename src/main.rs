use axum::Router;
use dotenvy::dotenv;
use tracing::info;

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
    let app = Router::new()
        .nest("/api/v1", routes::main())
        .layer(axum::middleware::from_fn(middlewares::logger::log_request));
    let port = std::env::var("APP_PORT")
        .expect(">>> APP_PORT NOT found!");
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
