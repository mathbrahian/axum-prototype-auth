use axum::{routing::{post,get}, Router};

use crate::AppState;

use super::handlers::{prelude::user_login, prelude::user_register, prelude::user_auth};

pub fn auth_router() -> Router<AppState> {
    Router::new()
        .route("/login", post(user_login))
        .route("/register", post(user_register))
        .route("/auth", get(user_auth))

}
