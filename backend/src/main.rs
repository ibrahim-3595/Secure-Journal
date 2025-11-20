mod auth;
mod db;
mod helpers;
mod models;

use axum::Server;
use auth::api::{login_api, signup_api};
use axum::{
    Router,
    extract::{Json, State},
    routing::post,
};
use db::DbPool;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};

pub struct AppState {
    pub db: DbPool,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let db = db::init_db().await?;

    let app_state = Arc::new(AppState { db });

    let _ = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any);
    let app: Router<_, _> = Router::new()
        .route("/api/signup", post(api_signup))
        .route("/api/login", post(api_login))
        .with_state(app_state);
        // .layer(cors);
    let addr = ([0, 0, 0, 0], 3000).into();
    println!("Server running on http://{}", addr);

    Server::bind(&addr).serve(app.into_make_service()).await?;

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
    match login_api(&state.db, &payload.username, &payload.password).await {
        Ok(Some(user)) => Json(AuthResponse {
            ok: true,
            message: format!("Logged in as {}", user.username),
        }),
        Ok(None) => Json(AuthResponse {
            ok: false,
            message: "Invalid credentials".into(),
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
    match signup_api(&state.db, &payload.username, &payload.password).await {
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
