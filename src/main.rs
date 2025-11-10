mod auth;
mod db;
mod menu;
mod models;

#[tokio::main]
async fn main() {
    println!("Welcome to secure Journal App..");

    let db = db::connect().await.expect("failed to connect to db");
    menu::main_menu(&db).await;
}
