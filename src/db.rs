use anyhow::Result;
use surrealdb::Surreal;
use surrealdb::engine::local::{Db, Mem};

pub async fn connect() -> Result<Surreal<Db>> {
    let db = Surreal::new::<Mem>(()).await?;
    println!("Connected to in-memory SurrealDB (Mem engine).");
    Ok(db)
}
