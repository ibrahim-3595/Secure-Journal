use colored::Colorize;
use dialoguer::Select;
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::Client;

use std::result::Result::Ok;
use crate::auth::entries::list_entries;
use crate::auth::delete::delete_user;
use crate::auth::entries::{delete_entry, list_users, new_entry, update_entry};
use crate::auth::login::login_flow;
use crate::auth::signup::signup_flow;
use crate::auth::entries::get_entries_for_user;

use crate::helpers::export::export_to_md;

use crate::models::models::User;

pub async fn main_menu(db: &Surreal<Client>) {
    let mut curr_usr: Option<User> = None;

    loop {
        let options = vec![
            "Login",                     //1
            "Create account",            //2
            "List Users",                //3x
            "Write a new journal entry", //4
            "View my journal entries",   //5
            "Update my journal entries", //6
            "Delete a journal entry",    //7
            "Delete my account",         //8
            "Export journal",            //9update
            "Logout",                    //10
            "Exit",                      //11
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
            //
            8 => {
                if let Some(user) = &curr_usr {            
                    let entries_res = get_entries_for_user(&db, user).await;
                    if let Ok(entries) = entries_res {
                        let file_name = format!(
                            "journal_export_{}_{}.md",
                            user.username,
                            chrono::Local::now().format("%Y-%m-%d_%H-%M")
                        );
            
                        if let Err(e) = export_to_md(&entries, &file_name) {
                            eprintln!("{}", format!("Export failed: {:?}", e).bright_red());
                        } else {
                            println!("{}", format!("Exported to {}", file_name).green());
                        }
                    } else {
                        eprintln!("{}", format!("Failed to fetch entries: {:?}", entries_res.err()).bright_red());
                    }
                } else {
                    println!("{}", "please login first..".red());
                }
                Ok(())
            }

            //
            9 => {
                curr_usr = None;
                println!("{}", "logged out..!".bright_yellow());
                Ok(())
            }
            10 => {
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
