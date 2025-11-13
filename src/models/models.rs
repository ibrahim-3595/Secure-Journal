use serde::{Deserialize, Serialize};

use surrealdb::Surreal;
use surrealdb::engine::remote::ws::Client;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub username: String,
    pub password: String,
    pub id: Option<surrealdb::RecordId>,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct JournalEntry {
    pub id: Option<surrealdb::RecordId>,
    pub user: String,
    pub title: String,
    pub content: String,
    //
    pub tags: Vec<String>, 
    pub created_at: String, 
    pub updated_at: String, 
}

// #[derive(Debug, Serialize, Deserialize, Clone)]
#[derive(Debug, Clone)]
pub struct _AppState {
    pub db: Surreal<Client>, 
    pub curr_usr: Option<User>, 
}
