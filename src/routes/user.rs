use axum::{
    routing::{delete, get, patch, post}, 
    Router
};
use crate::handlers::user;


pub fn main() -> Router {
    Router::new()
        .route("/register", post( user::register ))
        .route("/login", post( user::login ))
        .route("/refresh", get( user::refresh ))
        .route("/logout", post( user::logout ))
        .route("/update/info", patch( user::update_information ))
        .route("/update/pass", patch( user::update_password ))
        .route("/delete", delete( user::delete ))
}