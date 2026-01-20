use sqlx::SqlitePool;
use anyhow::Result;

pub type DbPool = SqlitePool;

pub async fn connect() -> Result<DbPool> {
    let pool = SqlitePool::connect("urlsqlite:///Users/ibrahimhaji/code/rust_programming/secure_journal/backend/data/journal.db").await?;
    Ok(pool)
}
