use colored::Colorize;
use dialoguer::Select;
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::Client;

use std::result::Result::Ok;

use crate::auth::login::login_flow;
use crate::auth::signup::signup_flow;
use crate::auth::delete::delete_user;
use crate::auth::entries::{delete_entry, list_entries, list_users, new_entry, update_entry};

use crate::models::models::User;

pub async fn main_menu(db: &Surreal<Client>) {
    let mut curr_usr: Option<User> = None;

    loop {
        let options = vec![
            "Login",
            "Create account",
            "List Users",
            "Write a new journal entry",
            "View my journal entries",
            "Update my journal entries", 
            "Delete a journal entry",
            "Delete my account",
            "Logout",
            "Exit",
        ];
        let selection = Select::new()
            .with_prompt("what would you like to do..?")
            .items(&options)
            .default(0)
            .interact()
            .unwrap();
        let result = match selection {
            0 => match login_flow(&db).await {
                Ok(Some(user)) => {
                    println!("{}", format!("logged in as {}", user.username).green());
                    curr_usr = Some(user);
                    Ok(())
                }
                Ok(None) => Ok(()),
                Err(e) => Err(e),
            },
            1 => signup_flow(&db).await,
            2 => list_users(&db).await,
            3 => {
                if let Some(user) = &curr_usr {
                    new_entry(&db, user).await
                } else {
                    println!("{}", "please login first..".red());
                    Ok(())
                }
            }
            4 => {
                if let Some(user) = &curr_usr {
                    list_entries(&db, user).await
                } else {
                    println!("{}", "please login first..".red());
                    Ok(())
                }
            }
            5 => {
                if let Some(user) = &curr_usr {
                    update_entry(&db, user).await
                } else {
                    println!("{}", "no entry to update..".red());
                    Ok(())
                }
            }
            6 => {
                if let Some(user) = &curr_usr {
                    delete_entry(&db, user).await
                } else {
                    println!("{}", "please login first..".red());
                    Ok(())
                }
            }
            7 => {
                if let Some(user) = &curr_usr {
                    let _ = delete_user(&db, user).await;
                    curr_usr = None;
                } else {
                    println!("{}", "please login first..".red());
                }
                Ok(())
            }
            8 => {
                curr_usr = None;
                println!("{}", "logged out..!".bright_yellow());
                Ok(())
            }
            9 => {
                println!("{}", "goodbye..!".cyan());
                return;
            }
            _ => Ok(()),
        };
        if let Err(e) = result {
            eprintln!("{}", format!("error: {:?}", e).bright_red());
        }
    }
}
