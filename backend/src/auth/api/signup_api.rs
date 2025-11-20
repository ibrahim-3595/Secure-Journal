use crate::db::DbPool;
use anyhow::{Result, anyhow};
use bcrypt::{hash, DEFAULT_COST};

pub async fn signup_api(pool: &DbPool, username: &str, password: &str) -> Result<()> {
    // Hash password
    let hashed = hash(password, DEFAULT_COST)?;

    // Insert user; rely on UNIQUE constraint to fail when username exists
    let res = sqlx::query("INSERT INTO users (username, password_hash) VALUES (?, ?)")
        .bind(username)
        .bind(&hashed)
        .execute(pool)
        .await;

    match res {
        Ok(_) => Ok(()),
        Err(e) => {
            // Optionally check for constraint error text to return nicer message
            Err(anyhow!(e))
        }
    }
}
