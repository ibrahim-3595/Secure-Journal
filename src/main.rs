use std::fmt::format;

use dialoguer::{Input, Select};
use rpassword::read_password;
use serde::{Deserialize, Serialize};
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct User {
    username: String,
    password: String,
    id: Option<surrealdb::RecordId>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct JournalEntry {
    id: Option<surrealdb::RecordId>,
    user: String,
    title: String,
    content: String,
}

#[tokio::main]
async fn main() {
    println!("Welcome to secure Journal app..");

    let db = Surreal::new::<Ws>("localhost:8000").await.unwrap();
    let _ = db
        .signin(Root {
            username: "root",
            password: "secret",
        })
        .await;
    db.use_ns("journal").use_db("database").await.unwrap();

    loop {
        let options = vec!["Login", "Create account", "Exit"];
        let selection = Select::new()
            .with_prompt("what would you like to do..")
            .items(&options)
            .default(0)
            .interact()
            .unwrap();
        match selection {
            0 => login_flow(&db),
            1 => login_flow(&db),
            2 => list_users(&db),
            _ => {
                println!("goodbye..!");
                break;
            }
        }.await.unwrap();
    }
}

async fn login_flow(db: &Surreal<Client>) -> surrealdb::Result<()> {
    let username = Input::<String>::new()
        .with_prompt("Username")
        .interact()
        .unwrap();

    println!("password");
    let password = read_password().unwrap();

    let query = format!("Select * from user where username ={:?}", username);
    let mut response = db.query(query).await?;
    let users: Option<Vec<User>> = response.take(0)?;

    if let Some(users) = users {
        if let Some(user) = users.first() {
            if user.password == password {
                println!("login successful! welcome {}.", username);
                // journal_menu().await?;
            } else {
                println!("incorrect password..plz try again");
            }
        } else {
            println!("user not found..");
        }
    } else {
        println!("no user data returned from databse..");
    }

    Ok(())
}

async fn list_users(db: Surreal<Client>) -> surrealdb::Result<()> {
    let mut response = db.query("select username from user").await?;
    let users: Vec<User> = response.take(0)?;
    println!("registered user..");
    for usr in users {
        println!("- {:?}", usr.username);
    }
    Ok(())
}

