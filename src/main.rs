mod auth;
mod db;
mod models;
mod common;
mod helpers;

use anyhow::Result;
use db::connect;

use crate::common::utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Welcome to Secure Journal App ;)");

    let db = connect().await.expect("Failed to connect to database");

    utils::main_menu(&db).await;
    
    Ok(())
}


//tomorrow's tasks: 
// integrate cloud and mem to enable user to save their entry once there exit the env
// integrate axum which integrates with surrealdb
// add a UI framework like dioxus/yew 