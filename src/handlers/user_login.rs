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

use chrono;

use jsonwebtoken::{encode, EncodingKey, Header};

use entity::user;

use crate::AppState;

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

    let now = chrono::Utc::now();
    let iat = now.timestamp() as usize;
    let exp = (now + chrono::Duration::minutes(60)).timestamp() as usize;

    let claims = Claims{
        sub: user_filter.id.to_string(),
        exp,
        iat,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret("secret".as_ref())
    ).map_err(|e| {
        let error_jwt = serde_json::json!({
            "message": format!("jwt token creation error: {}", e),
        });
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(error_jwt))
    })
    .unwrap();

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