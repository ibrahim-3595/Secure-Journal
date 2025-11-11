use colored::Colorize;
use dialoguer::Select;
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::Client;

use std::result::Result::Ok;

use crate::auth::{
    delete_entry, delete_user, list_entries, list_users, login_flow, new_entry, signup_flow,
};
use crate::models::User;

pub async fn main_menu(db: &Surreal<Client>) {
    let mut curr_usr: Option<User> = None;

    loop {
        let options = vec![
            "Login",
            "Create account",
            "List Users",
            "Write a new journal entry",
            "View my journal entries",
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
                    delete_entry(&db, user).await
                } else {
                    println!("{}", "please login first..".red());
                    Ok(())
                }
            }
            6 => {
                if let Some(user) = &curr_usr {
                    delete_user(&db, user).await;
                    curr_usr = None;
                } else {
                    println!("{}", "please login first..".red());
                }
                Ok(())
            }
            7 => {
                curr_usr = None;
                println!("{}", "logged out..!".bright_yellow());
                Ok(())
            }
            8 => {
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
