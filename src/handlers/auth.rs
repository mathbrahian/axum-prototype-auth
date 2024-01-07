use axum::{
    extract::Json,
    http::{StatusCode, Request, header},
    response::IntoResponse,
};

use jsonwebtoken::{decode, DecodingKey, Validation};

use serde::Deserialize;

pub async fn user_auth<B>(
    req: Request<B>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {

    let token = req.headers()
    .get(header::AUTHORIZATION)
    .and_then(|auth_header| auth_header.to_str().ok())
    .and_then(|auth_value| {
        if auth_value.starts_with("Bearer ") {
            Some(auth_value[7..].to_owned())
        } else {
            None
        }
    });

    let token = token.ok_or_else(|| {
        let json_error = serde_json::json!({
            "status": "fail",
            "message": "Invalid token".to_string(),
        });
        (StatusCode::UNAUTHORIZED, Json(json_error))
    })?;

    let decoded = decode::<Claims>(
        &token,
        &DecodingKey::from_secret("secret".as_ref()),
        &Validation::default(),
    )
    .map_err(|_| {
        let error_decode = serde_json::json!({
            "message": "Invalid token".to_string()
        });
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(error_decode))
    })?
    .claims;

    let decode_result = serde_json::json!({"id": &decoded.sub});


    return Ok((StatusCode::ACCEPTED, Json(decode_result)));
}


#[derive(Debug, Deserialize)]
pub struct Claims {
    sub: String,
}