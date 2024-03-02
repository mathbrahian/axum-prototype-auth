use axum::{
    extract::Json,
    http::StatusCode,
};

use uuid::Uuid;

use serde::{
    Deserialize,
    Serialize
};


use jsonwebtoken::{encode, EncodingKey, Header};
use serde_json;


pub fn create_token(user_id: Uuid) -> String{

        let now = chrono::Utc::now();
    let iat = now.timestamp() as usize;
    let exp = (now + chrono::Duration::minutes(60)).timestamp() as usize;

    let claims = Claims{
        sub: user_id.to_string(),
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

    return token

}

#[derive(Serialize, Deserialize)]
pub struct Claims {
    sub: String,
    iat: usize,
    exp: usize,
}