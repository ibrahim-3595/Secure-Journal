mod auth;
mod db;
mod menu;
mod models;

#[tokio::main]
async fn main() {
    println!("Welcome to secure Journal App ;)");

    let db = db::connect().await.expect("failed to connect to db");
    menu::main_menu(&db).await;
}

//tomorrow's tasks:
// validation for username and password eg if username.isempty and password < 8 chars 
// spinners & progress indicators using indicatif
// add delete method to delete a user or a journal entry 
// organized menu (optional)
// 
// database and storage enchancements like timestamps, tags, soft delete
// modules for entries.rs & utils.rs
// export journal to .md or .pdf using pdf crate
// sync journal enteries with cloud storage or self-hosted server
// integrate axum which integrates with surrealdb
// add a UI framework like dioxus/yew 
// 