use dialoguer::Select;
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::Client;

use crate::auth::{list_entries, list_users, login_flow, new_entry, signup_flow};

pub async fn main_menu(db: &Surreal<Client>) {
    loop {
        let options = vec![
            "Login",
            "Create account",
            "List Users",
            "Write a new journal entry",
            "View my journal entries",
            "Exit",
        ];
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
            3 => new_entry(&db, ), //queue
            4 => list_entries(&db, ), //queue
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
