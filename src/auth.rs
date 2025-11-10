use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::{PasswordHash, SaltString};
use argon2::{Argon2, PasswordHasher, PasswordVerifier};

use anyhow::{Ok, Result};
use colored::*;
use dialoguer::Input;
use rpassword::read_password;
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::Client;

use crate::models::{JournalEntry, User};

pub async fn signup_flow(db: &Surreal<Client>) -> Result<()> {
    let username = Input::<String>::new()
        .with_prompt("choose a username..")
        .interact()
        .map_err(|e| anyhow::anyhow!("input error: {e}"))?;
    println!("choose a password..");
    let password = read_password().unwrap();

    //confirm pass
    println!("confirm password..");
    let confirm_pass = read_password().unwrap();
    if confirm_pass != password {
        println!("{}", "passwords do not match..".bright_red());
        return Ok(());
    }

    //hashing..
    let mut rng = OsRng;
    let salt = SaltString::generate(&mut rng);
    let hashed = Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .unwrap()
        .to_string();

    //insert..
    let _: Option<User> = db
        .create("user")
        .content(User {
            username,
            password: hashed,
            id: None,
        })
        .await
        .map_err(|e| anyhow::anyhow!("input error: {e}"))?;
    println!("{}", "account created successfully..".green());

    Ok(())
}

pub async fn login_flow(db: &Surreal<Client>) -> Result<Option<User>> {
    let username = Input::<String>::new()
        .with_prompt("username")
        .interact()
        .unwrap();

    println!("password:");
    let password = read_password().unwrap();

    let query = format!("select * from user where username ={:?}", username);
    let mut response = db.query(query).await?;
    let users: Option<Vec<User>> = response.take(0)?;

    if let Some(users) = users {
        if let Some(user) = users.first() {
            let parsed_hash = PasswordHash::new(&user.password)?;
            if Argon2::default()
                .verify_password(password.as_bytes(), &parsed_hash)
                .is_ok()
            {
                println!("{}", format!("login successful! welcome, {}!", user.username).green());
                return Ok(Some(user.clone()));
            } else {
                println!("{}", "incorrect password..".red());
                return Ok(None);
            }
        } else {
            println!("{}", "no such user found..".bright_red());
            Ok(None)
        }
    } else {
        println!("{}", "no data was fetched from db..".bright_red());
        Ok(None)
    }
}

pub async fn list_users(db: &Surreal<Client>) -> Result<()> {
    let mut response = db.query("select * from user").await?;
    let users: Vec<User> = response.take(0)?;
    println!("{}", "registered user..".bright_green());
    for usr in users {
        println!("- {:?}", usr.username);
    }

    Ok(())
}

pub async fn new_entry(db: &Surreal<Client>, user: &User) -> Result<()> {
    let title = Input::<String>::new()
        .with_prompt("title")
        .interact()
        .unwrap();
    let content = Input::<String>::new()
        .with_prompt("content")
        .interact()
        .unwrap();

    let _: Option<JournalEntry> = db
        .create("entry")
        .content(JournalEntry {
            id: None,
            user: user.username.clone(),
            title,
            content,
        })
        .await?;
    println!("{}", "journal entry has been saved..".green());

    Ok(())
}

pub async fn list_entries(db: &Surreal<Client>, user: &User) -> Result<()> {
    let query = format!("select * from entry where user = {:?}", user.username);
    let mut resp = db.query(query).await?;
    let entries: Vec<JournalEntry> = resp.take(0)?;

    if entries.is_empty() {
        println!("{}", "no entires found for {user.username}".red());
    } else {
        println!("your journal entires: ");
        for (i, entry) in entries.iter().enumerate() {
            println!("{}. {} - {}", i + 1, entry.title, entry.content);
        }
    }

    Ok(())
}
