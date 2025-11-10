use dialoguer::Select;
use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;

use crate::auth::{signup_flow, login_flow, list_users};

pub async fn main_menu(db: &Surreal<Client>) {
    loop {
        let options = vec!["Login", "Create account", "Exit"];
        let selection = Select::new()
            .with_prompt("what would you like to do..")
            .items(&options)
            .default(0)
            .interact()
            .unwrap();
        let result = match selection {
            0 => login_flow(&db).await,
            1 => signup_flow(&db).await,
            2 => list_users(&db).await,
            _ => {
                println!("goodbye..!");
                break;
            }
        };
        if let Err(e) = result {
            eprintln!(" error : {:?}", e);
        }
    }
}