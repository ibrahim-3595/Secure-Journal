use crate::db::DbPool;
use crate::models::models::User;

use anyhow::{Result, bail};
use bcrypt::verify;
use sqlx::Row;

/// Login API: returns Some(User) when successful, None when credentials fail.
pub async fn login_api(pool: &DbPool, username: &str, password: &str) -> Result<Option<User>> {
    let row = sqlx::query(
        "SELECT id, username, password_hash 
         FROM users 
         WHERE username = ?"
    )
    .bind(username)
    .fetch_optional(pool)
    .await?;

    let Some(row) = row else {
        // username not found
        return Ok(None);
    };

    let id: i64 = row.get("id");
    let username: String = row.get("username");
    let password_hash: String = row.get("password_hash");

    // verify password with bcrypt
    let correct = verify(password, &password_hash)
        .map_err(|e| anyhow::anyhow!("bcrypt verification error: {}", e))?;

    if !correct {
        return Ok(None);
    }

    Ok(Some(User {
        id,
        username,
        password_hash,
        password: None, // never expose real password
    }))
}
