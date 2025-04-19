use axum::Router;
use tracing::info;

mod routes;
mod handlers;
mod middlewares;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let app = Router::new()
        .nest("/api/v1", routes::main())
        .layer(axum::middleware::from_fn(middlewares::logger::log_request));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .expect(">>> Can NOT create the listener!");
    info!("App running on: {}", listener.local_addr().unwrap());
    axum::serve(listener, app)
        .await
        .expect(">>> Axum can NOT serve us!");
}
