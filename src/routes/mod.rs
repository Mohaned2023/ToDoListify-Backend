use axum::Router;

mod user;

pub fn main() -> Router {
    Router::new()
        .nest("/user", user::main())
}