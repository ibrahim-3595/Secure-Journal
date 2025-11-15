use colored::Colorize;
use dialoguer::Select;

use surrealdb::engine::local::Db;
use surrealdb::Surreal;
use std::result::Result::Ok;

use crate::auth::delete::delete_user;
use crate::auth::entries::get_entries_for_user;
use crate::auth::entries::list_entries;
use crate::auth::entries::{delete_entry, list_users, new_entry, update_entry};
use crate::auth::login::login_flow;
use crate::auth::signup::signup_flow;
use crate::helpers::export::export_to_md;
use crate::helpers::import::import_md;
use crate::models::models::User;

pub async fn main_menu(db: &Surreal<Db>) {
    let mut curr_usr: Option<User> = None;

    loop {
        let options = vec![
            "Login",                     //0
            "Create account",            //1
            "List Users",                //2
            "Write a new journal entry", //3
            "View my journal entries",   //4
            "Update my journal entries", //5
            "Delete a journal entry",    //6
            "Delete my account",         //7
            "Export journal",            //8
            "Import journal",            //9
            "Logout",                    //10
            "Exit",                      //_
        ];
        let selection = Select::new()
            .with_prompt("What would you like to do?")
            .items(&options)
            .default(0)
            .interact()
            .unwrap();
        
        let result = match selection {
            0 => {
                match login_flow(db).await {
                    Ok(Some(user)) => {
                        curr_usr = Some(user.clone());
                        println!("Logged in as {}", user.username);
                        Ok(())
                    }
                    Ok(None) => {
                        println!("Login failed");
                        Ok(())
                    }
                    Err(e) => Err(e),
                }
            }
            1 => {
                signup_flow(db).await
            }
            2 => list_users(&db).await,
            3 => {
                if let Some(user) = &curr_usr {
                    new_entry(&db, user).await
                } else {
                    println!("{}", "Please login first".red());
                    Ok(())
                }
            }
            4 => {
                if let Some(user) = &curr_usr {
                    list_entries(&db, user).await
                } else {
                    println!("{}", "Please login first".red());
                    Ok(())
                }
            }
            5 => {
                if let Some(user) = &curr_usr {
                    update_entry(&db, user).await
                } else {
                    println!("{}", "No entry to update".red());
                    Ok(())
                }
            }
            6 => {
                if let Some(user) = &curr_usr {
                    delete_entry(&db, user).await
                } else {
                    println!("{}", "Please login first".red());
                    Ok(())
                }
            }
            7 => {
                if let Some(user) = &curr_usr {
                    let _ = delete_user(&db, user).await;
                    curr_usr = None;
                } else {
                    println!("{}", "Please login first".red());
                }
                Ok(())
            }
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
                        eprintln!(
                            "{}",
                            format!("Failed to fetch entries: {:?}", entries_res.err())
                                .bright_red()
                        );
                    }
                } else {
                    println!("{}", "Please login first".red());
                }
                Ok(())
            }
            9 => {
                println!("{}", "Enter path to .md file:".cyan());
                let mut path = String::new();
                let _ = std::io::stdin().read_line(&mut path);
                let path = path.trim();

                match import_md(path) {
                    Ok(entries) => {
                        for e in entries {
                            println!("{}", format!("Imported: {}", e.title).green());
                        }
                    }
                    Err(e) => eprintln!("{}", format!("Import failed: {:?}", e).red()),
                }

                Ok(())
            }
            10 => {
                curr_usr = None;
                println!("{}", "Logged Out!!".bright_yellow());
                Ok(())
            }
            11 => {
                println!("{}", "Good-Bye..! ;)".cyan());
                return;
            }
            _ => Ok(()),
        };
        
        if let Err(e) = result {
            eprintln!("{}", format!("error: {:?}", e).bright_red());
        }
    }
}