use std::sync::Arc;
use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};

use crate::AppState;
use crate::signup_api;

#[derive(Deserialize)]
pub struct AuthRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct AuthResponse {
    pub ok: bool,
    pub message: String,
}

pub async fn api_signup(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<AuthRequest>,
) -> Json<AuthResponse> {
    match signup_api(&state.db, &payload.username, &payload.password).await {
        Ok(_) => Json(AuthResponse { ok: true, message: "Signup successful".into() }),
        Err(e) => Json(AuthResponse { ok: false, message: e.to_string() }),
    }
}
