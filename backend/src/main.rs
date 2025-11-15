mod auth;
mod common;
mod db;
mod helpers;
mod models;

use axum::{Json, Router, extract::State, routing::post};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tower_http::cors::CorsLayer;

use crate::auth::login::login_flow;
use crate::auth::signup::signup_flow;
use crate::db::connect;

#[derive(Clone)]
struct AppState {
    db: surrealdb::Surreal<surrealdb::engine::remote::ws::Client>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let db = connect().await?;
    let state = Arc::new(AppState { db });

    let app = Router::new()
        .route("/api/login", post(api_login))
        .route("/api/signup", post(api_signup))
        .layer(CorsLayer::permissive())
        .with_state(state);

    println!("Server running http://127.0.0.1:3001");
    axum::Server::bind(&"127.0.0.1:3001".parse()?)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

#[derive(Deserialize)]
struct AuthRequest {
    username: String,
    password: String,
}

#[derive(Serialize)]
struct AuthResponse {
    ok: bool,
    message: String,
}

async fn api_login(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<AuthRequest>,
) -> Json<AuthResponse> {
    match login_flow(&state.db, &payload.username, &payload.password).await {
        Ok(Some(user)) => Json(AuthResponse {
            ok: true,
            message: format!("Logged in as {}", user.username),
        }),
        Ok(None) => Json(AuthResponse {
            ok: false,
            message: "Invalid credentials".to_string(),
        }),
        Err(e) => Json(AuthResponse {
            ok: false,
            message: format!("Error: {}", e),
        }),
    }
}

async fn api_signup(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<AuthRequest>,
) -> Json<AuthResponse> {
    match signup_flow(&state.db, &payload.username, &payload.password).await {
        Ok(()) => Json(AuthResponse {
            ok: true,
            message: "Signup successful".into(),
        }),
        Err(e) => Json(AuthResponse {
            ok: false,
            message: format!("Error: {}", e),
        }),
    }
}
