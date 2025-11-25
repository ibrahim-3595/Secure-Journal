use crate::db::DbPool;
use crate::models::models::User;

use anyhow::Result;
use sqlx::Row;

use argon2::{Argon2, PasswordHash, PasswordVerifier};

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
        return Ok(None);
    };

    let id: i64 = row.get("id");
    let username: String = row.get("username");
    let stored_hash: String = row.get("password_hash");

    let parsed = PasswordHash::new(&stored_hash)
        .map_err(|e| anyhow::anyhow!("Invalid hash in DB: {}", e))?;

    let ok = Argon2::default()
        .verify_password(password.as_bytes(), &parsed)
        .is_ok();

    if !ok {
        return Ok(None);
    }

    Ok(Some(User {
        id,
        username,
        password_hash: stored_hash,
    }))
}
