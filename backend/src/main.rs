mod auth;
mod common;
mod db;
mod helpers;
mod models;
mod router;

use common::utils::main_menu;
use db::DbPool;

use std::sync::Arc;
use axum::Server;

use crate::router::create_router;

pub struct AppState {
    pub db: DbPool,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let db = db::connect().await?;
    let state = Arc::new(AppState { db });

    tokio::spawn(start_server(state.clone()));

    main_menu(&state.db).await;

    Ok(())
}

pub async fn start_server(state: Arc<AppState>) {
    let app = create_router(state);

    println!("HTTP API running on http://127.0.0.1:8000");

    Server::bind(&"0.0.0.0:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
