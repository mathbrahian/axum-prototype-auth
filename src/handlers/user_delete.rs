use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::IntoResponse,
};

use uuid::Uuid;


use crate::AppState;

use serde::{
    Deserialize,
    Serialize
};

use entity::user;

use sea_orm::{
    ColumnTrait,
    EntityTrait,
    QueryFilter
};


use serde_json;

pub async fn user_delete(
    State(state): State<AppState>,
    Json(payload): Json<GetUser>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let _user_filter = user::Entity::delete_many()
    .filter(user::Column::Id.eq(payload.id)).exec(&state.conn).await
    .map_err(|e| {
        let error_database = serde_json::json!({
            "message": format!("Database error: {}", e),
        });
        (StatusCode::INTERNAL_SERVER_ERROR, Json(error_database))
    });

    let authenticate = serde_json::json!({"delete": "sucess"});

    return Ok((StatusCode::ACCEPTED, Json(authenticate)));
}

#[derive(Serialize, Deserialize)]
pub struct GetUser {
    id: Uuid,
}