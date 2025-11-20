use crate::models::models::User;
use crate::db::DbPool;
use anyhow::Result;
use bcrypt::verify;
use sqlx::Row;

pub async fn login_api(pool: &DbPool, username: &str, password: &str) -> Result<Option<User>> {
    // Query user by username
    let row = sqlx::query("SELECT id, username, password_hash FROM users WHERE username = ?")
        .bind(username)
        .fetch_optional(pool)
        .await?;

    if let Some(row) = row {
        let id: i64 = row.get("id");
        let username: String = row.get("username");
        let password_hash: String = row.get("password_hash");

        // verify password
        if verify(password, &password_hash)? {
            let user = User {
                id: Some(id),
                username,
                password_hash,
                password: None, 
            };
            Ok(Some(user))
        } else {
            Ok(None)
        }
    } else {
        Ok(None)
    }
}
