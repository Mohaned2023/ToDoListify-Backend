use axum::{
    middleware, 
    routing::{
        delete, 
        get, 
        patch, 
        post
    }, 
    Router
};

use crate::{
    middlewares,
    handlers
};

pub fn main() -> Router {
    Router::new()
        .route("/", get(handlers::task::get_all))
        .route("/create", post(handlers::task::create))
        .route("/update/{id}", patch(handlers::task::update))
        .route("/delete/{id}", delete(handlers::task::delete))
        .route_layer(middleware::from_fn(middlewares::auth::auth_guard))
}