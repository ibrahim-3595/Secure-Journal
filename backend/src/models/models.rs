use serde::{Deserialize, Serialize};
use surrealdb::Surreal;
use surrealdb::engine::local::Db;
use surrealdb::sql::Thing;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub username: String,
    pub password: Option<String>,
    pub id: Option<i64>,
    //
    pub password_hash: String, 
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct JournalEntry {
    pub id: Option<Thing>,
    pub user: String,
    pub title: String,
    pub content: String,
    pub tags: Vec<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone)]
pub struct _AppState {
    pub db: Surreal<Db>, 
    pub curr_usr: Option<User>, 
}
