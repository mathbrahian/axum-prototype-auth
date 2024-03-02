use argon2::{
    password_hash::{
        PasswordHash,
        PasswordVerifier
    },
    Argon2,
};

use axum::{
    extract::{
        Json,
        State
    },
    http::StatusCode,
    response::IntoResponse,
};

use serde::{
    Deserialize,
    Serialize
};

use sea_orm::{
    ColumnTrait,
    EntityTrait,
    QueryFilter
};

use serde_json;

use entity::user;


use crate::{utils, AppState};

use utils::create_token::create_token;

pub async fn user_login(
    State(state): State<AppState>,
    Json(payload): Json<UserDeserialize>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let user_filter = user::Entity::find()
        .filter(user::Column::Email.eq(payload.email))
        .one(&state.conn)
        .await
        .map_err(|e| {
            let error_database = serde_json::json!({
                "message": format!("Database error: {}", e),
            });
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_database))
        })?
        .ok_or_else(|| {
            let error_email = serde_json::json!({
                "message": "This email is not registered",
            });
            (StatusCode::BAD_REQUEST, Json(error_email))
        })?;

    let parsed_hash = PasswordHash::new(&user_filter.password).unwrap();

    let is_valid = Argon2::default()
        .verify_password(payload.password.as_bytes(), &parsed_hash)
        .is_ok();

    if !is_valid {
        let error_is_valid_password = serde_json::json!({
            "message": "Incorrect password",
        });
        return Err((StatusCode::BAD_REQUEST, Json(error_is_valid_password)));
    };

    let token = create_token(user_filter.id);

    let authenticate = serde_json::json!({"token": token});

    return Ok((StatusCode::ACCEPTED, Json(authenticate)));
}

#[derive(Deserialize)]
pub struct UserDeserialize {
    email: String,
    password: String,
}

#[derive(Serialize, Deserialize)]
pub struct Claims {
    sub: String,
    iat: usize,
    exp: usize,
}