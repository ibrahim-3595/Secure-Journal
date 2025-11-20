use anyhow::Result;
use std::fs;
use surrealdb::Surreal;
use surrealdb::engine::local::{Db, Mem, Sqlite};

use crate::models::models::User;

use surrealdb::Surreal;
use surrealdb::engine::local::Sqlite;

pub async fn connect() -> Result<Surreal<Sqlite>> {
    let db = Surreal::new::<Sqlite>("journal.db").await?;

    db.use_ns("app").use_db("journal").await?;

    println!("Connected to local SQLite SurrealDB");

    Ok(db)
}


pub async fn save_users(users: &[User]) -> Result<()> {
    let data = serde_json::to_string_pretty(&users)?;
    fs::write("users.json", data)?;
    println!("Users saved to users.json");
    Ok(())
}
pub async fn load_users() -> Result<Vec<User>> {
    let data = fs::read_to_string("users.json").unwrap_or_else(|_| "[]".into());
    let users: Vec<User> = serde_json::from_str(&data)?;
    Ok(users)
}
