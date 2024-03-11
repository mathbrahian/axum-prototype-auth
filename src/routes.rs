use axum::{routing::{post,get,delete}, Router};

use crate::AppState;

use super::handlers::{prelude::user_login, prelude::user_register, prelude::user_auth, prelude::user_delete};

pub fn auth_router() -> Router<AppState> {
    Router::new()
        .route("/login", post(user_login))
        .route("/register", post(user_register))
        .route("/delete", delete(user_delete))
        .route("/auth", get(user_auth))

}
