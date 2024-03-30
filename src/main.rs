use axum::{routing::get, Router};

use sea_orm::{Database, DatabaseConnection};

use dotenv::dotenv;

use routes::auth_router;

mod handlers;
mod routes;
mod utils;

use tower_http::trace::{self, TraceLayer};
use tracing::Level;

#[derive(Clone)]
pub struct AppState {
    conn: DatabaseConnection,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    tracing_subscriber::fmt()
    .with_target(false)
    .compact()
    .init();

    let database_url = std::env::var("DATABASE_AUTH_URL")
        .unwrap();

    let conn = Database::connect(database_url)
        .await
        .expect("Database connection failed");

    let state = AppState { conn };

    let user_router = Router::new()
        .route("/ping", get(ping))
        .nest("/", auth_router());

    let app = Router::new().nest("/auth", user_router).with_state(state).layer(
        TraceLayer::new_for_http()
            .make_span_with(trace::DefaultMakeSpan::new()
                .level(Level::INFO))
            .on_response(trace::DefaultOnResponse::new()
                .level(Level::INFO)),

    );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:80").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn ping() -> String {
    return "ping".to_string();
}
