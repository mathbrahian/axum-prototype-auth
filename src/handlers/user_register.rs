use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::IntoResponse,
};

use uuid::Uuid;

use sha256::digest;

use crate::{utils, AppState};

use serde::{
    Deserialize,
    Serialize
};

use entity::user;

use sea_orm::{
    ActiveModelTrait,
    ColumnTrait,
    EntityTrait,
    QueryFilter,Set
};


use serde_json;

use utils::create_token::create_token;

pub async fn user_register(
    State(state): State<AppState>,
    Json(payload): Json<UserDeserialize>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let email_is_registered = user::Entity::find()
        .filter(user::Column::Email.eq(payload.email.clone()))
        .one(&state.conn)
        .await
        .map_err(|e| {
            let error_database = serde_json::json!({
                "message": format!("Database error: {}", e),
            });
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_database))
        })
        .unwrap()
        .is_some();

    if email_is_registered {
        let error_email_is_registered = serde_json::json!({
            "message": "this email is registered",
        });
        return Err((StatusCode::BAD_REQUEST, Json(error_email_is_registered)));
    }

    let hashed_password = digest(payload.password);

    let user_model = user::ActiveModel {
        email: Set(payload.email),
        password: Set(hashed_password),
        id: Set(payload.id),
        salt: Set("salt".to_string()),
    };

    let user_model_insert: user::Model = user_model
        .insert(&state.conn)
        .await
        .map_err(|e| {
            let error_database = serde_json::json!({
                "message": format!("Database error: {}", e),
            });
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_database))
        })
        .unwrap();

        let token = create_token(user_model_insert.id);

        let authenticate = serde_json::json!({"auth": {"token": token, "email": user_model_insert.email}});

    Ok((StatusCode::CREATED, Json(authenticate)))
}

#[derive(Deserialize)]
pub struct UserDeserialize {
    email: String,
    password: String,
    id: Uuid,
}

#[derive(Serialize, Deserialize)]
pub struct GetUser {
    id: Uuid,
}