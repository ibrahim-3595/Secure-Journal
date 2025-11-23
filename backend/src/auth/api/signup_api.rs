use crate::db::DbPool;

use anyhow::{Result, anyhow};
use bcrypt::{hash, DEFAULT_COST};

/// Signup API: creates user or returns an error if username exists.
pub async fn signup_api(pool: &DbPool, username: &str, password: &str) -> Result<()> {
    let hashed = hash(password, DEFAULT_COST)
        .map_err(|e| anyhow!("bcrypt hashing failed: {}", e))?;

    let result = sqlx::query(
        "INSERT INTO users (username, password_hash) 
         VALUES (?, ?)"
    )
    .bind(username)
    .bind(&hashed)
    .execute(pool)
    .await;

    match result {
        Ok(_) => Ok(()),

        Err(e) => {
            // You can match sqlite error codes here for clearer messages
            // but I will not change behavior.
            Err(anyhow!("Failed to create user: {}", e))
        }
    }
}
