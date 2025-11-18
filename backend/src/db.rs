use anyhow::Result;
use std::fs::File;
use surrealdb::Surreal;
use surrealdb::engine::local::{Db, Mem, RocksDb};

pub async fn connect() -> Result<Surreal<Db>> {
    // let db = Surreal::new::<Mem>(()).await?;
    let db =
        Surreal::new::<RocksDb>("/home/ibrahim/code/rust_programming/secure_journal/mydb").await?;

    db.use_ns("app").use_db("journal").await?;

    println!("Connected to file-backend SurrealDB");
    Ok(db)
}
