use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { "hello world!" } ));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .expect(">>> Can NOT create the listener!");
    println!(">>> App running on: {}", listener.local_addr().unwrap());
    axum::serve(listener, app)
        .await
        .expect(">>> Axum can NOT serve us!");
}
