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
// create the working dir like this..
// src/
// ├── auth/
// │   ├── mod.rs          # re-exports everything
// │   ├── login.rs
// │   ├── signup.rs
// │   ├── delete.rs
// │   └── validate.rs
// ├── db/
// │   ├── mod.rs
// │   └── connection.rs
// ├── common/
// │   ├── error.rs
// │   └── utils.rs
// organized menu (optional)
// 
// database and storage enchancements like timestamps, tags, soft delete
// export journal to .md or .pdf using pdf crate
// sync journal enteries with cloud storage or self-hosted server
// integrate axum which integrates with surrealdb
// add a UI framework like dioxus/yew 
// 