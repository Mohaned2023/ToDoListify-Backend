use axum::Router;

mod user;
mod task;

pub fn main() -> Router {
    Router::new()
        .nest("/user", user::main())
        .nest("/task", task::main())
}