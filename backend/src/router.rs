use std::sync::Arc;
use axum::{Router, routing::post};

use crate::AppState;
use crate::auth::http::{
    login_handler::api_login,
    signup_handler::api_signup,
};

pub fn create_router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/login", post(api_login))
        .route("/api/signup", post(api_signup))
        .with_state(state)
}
