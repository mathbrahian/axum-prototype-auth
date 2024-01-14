use axum::{routing::get, Router};

use sea_orm::{Database, DatabaseConnection};

use routes::auth_router;

mod handlers;
mod routes;

#[derive(Clone)]
pub struct AppState {
    conn: DatabaseConnection,
}

#[tokio::main]
async fn main() {
    let conn = Database::connect("postgres://postgres:postgres@auth_db:5432/postgres")
        .await
        .expect("Database connection failed");

    let state = AppState { conn };

    let user_router = Router::new()
        .route("/ping", get(ping))
        .nest("/api", auth_router());

    let app = Router::new().nest("/user", user_router).with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn ping() -> String {
    return "ping".to_string();
}
