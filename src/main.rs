use surrealdb::engine::remote::ws::Ws;
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;
use serde::{Serialize, Deserialize};
use dialoguer::{Input, Select};
use rpassword::read_password;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct User {
    username: String,
    password: String,
    id: Option<surrealdb::RecordId>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct JournalEntry {
    id: Option<surrealdb::RecordId>,
    user: String,
    title: String,
    content: String, 
}

#[tokio::main]
async fn main() {
    println!("Wrlcome to secure Journal app..");
}