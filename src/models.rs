use serde::{Deserialize, Serialize};
use surrealdb::RecordId;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub username: String,
    pub password: String,
    pub id: Option<surrealdb::RecordId>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JournalEntry {
    pub id: Option<surrealdb::RecordId>,
    pub user: String,
    pub title: String,
    pub content: String,
}
