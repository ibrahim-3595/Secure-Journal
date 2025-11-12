use anyhow::{Ok, Result};
use colored::*;
use dialoguer::Confirm;
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::Client;

use crate::models::models::User;

pub async fn delete_user(db: &Surreal<Client>, user: &User) -> Result<()> {
    let confirm = Confirm::new()
        .with_prompt(format!(
            "are you sure you wanna delete '{}' and all their entries..?",
            user.username
        ))
        .default(false)
        .interact()
        .unwrap();
    if !confirm {
        println!("{}", "deletion cancelled..".yellow());
        return Ok(());
    }

    //del usr
    let query_user = format!("delete user where username = {:?}", user.username);
    db.query(query_user).await?;

    //del journal entry
    let query_entries = format!("delete entry where user = {:?}", user.username);
    db.query(query_entries).await?;

    println!(
        "{}",
        "user and all their entries are deleted successfully..".green()
    );

    Ok(())
}
