use anyhow::Result;
use sqlx::{Pool, Sqlite, SqlitePool};

pub type DbPool = Pool<Sqlite>;

pub async fn init_db() -> Result<DbPool> {
    // let pool = SqlitePool::connect("sqlite:journal.db").await?;
    // let pool = SqlitePool::connect("/home/ibrahim/code/rust_programming/secure_journal/journal.db").await?;
    std::fs::create_dir_all("data")?;
    let pool = SqlitePool::connect("data/journal.db").await?;
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            username TEXT UNIQUE NOT NULL,
            password_hash TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS entries (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            user_id INTEGER NOT NULL,
            content TEXT NOT NULL,
            created_at TEXT NOT NULL,
            FOREIGN KEY(user_id) REFERENCES users(id)
        );
        "#,
    )
    .execute(&pool)
    .await?;

    Ok(pool)
}

// pub async fn save_db(db: &Surreal<Db>) -> Result<()> {
//     let export = db.export(()).await?;
//     let _ = fs::write("db_backup.json", export)?;
//     Ok(())
// }
// pub async fn load_users() -> Result<Vec<User>> {
//     let data = fs::read_to_string("users.json").unwrap_or_else(|_| "[]".into());
//     let users: Vec<User> = serde_json::from_str(&data)?;
//     Ok(users)
// }
