use axum::{
    middleware, routing::{delete, get, patch, post}, Router
};
use crate::{handlers::user, middlewares};


pub fn main() -> Router {
    Router::new()
        .route("/register", post( user::register ))
        .route("/login", post( user::login ))
        .route(
            "/refresh", 
            get( user::refresh )
                .route_layer(middleware::from_fn(middlewares::auth::auth_guard))
        )
        .route("/logout", post( user::logout ))
        .route("/update/info", patch( user::update_information ))
        .route("/update/pass", patch( user::update_password ))
        .route("/delete", delete( user::delete ))
}