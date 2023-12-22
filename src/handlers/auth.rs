use axum::{
    extract::{
        Json,
        State
    },
    http::StatusCode,
    response::IntoResponse,
};

use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};

use serde::Deserialize;

use crate::AppState;

pub async fn user_auth(
    // State(state): State<AppState>,
    Json(payload): Json<TokenDeserialize>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    // let new_token = serde_json::json!({"token": payload.token});

    let decoded = decode::<Claims>(
        &payload.token,
        &DecodingKey::from_secret("secret".as_ref()),
        &Validation::default(),
    )
    .map_err(|_| {
        let error_decode = serde_json::json!({
            "message": "Invalid token".to_string(),
        });
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(error_decode))
    })?.claims;

    let decode_result = serde_json::json!({"id": &decoded.sub});


    return Ok((StatusCode::ACCEPTED, Json(decode_result)));
}

#[derive(Deserialize)]
pub struct TokenDeserialize {
    token: String,
}

#[derive(Debug, Deserialize)]
pub struct Claims {
    sub: String,
    _iat: usize,
    _exp: usize,
}