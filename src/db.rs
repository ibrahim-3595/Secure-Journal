use surrealdb::Surreal;
use surrealdb::engine::local::{Db, File};
use anyhow::Result;

pub async fn connect() -> Result<Surreal<Db>> {
    let db = Surreal::new::<File>("secure_journal.db").await?;
    println!("Connected to local SurrealDB file database..");
    Ok(db)
}






//reduntant code needed for later...dont del
// use surrealdb::Surreal;
// use surrealdb::engine::remote::ws::{Client, Ws};
// use surrealdb::opt::auth::Root;

// pub async fn connect() -> Result<Surreal<Db>> {
//     let db = Surreal::new::<File>("secure_journal.db").await.unwrap();
//     // let _ = db
//     //     .signin(Root {
//     //         username: "root",
//     //         password: "secret",
//     //     })
//     //     .await;
//     // db.use_ns("journal").use_db("database").await.unwrap();
    
//     Ok(db)
// }
// 
