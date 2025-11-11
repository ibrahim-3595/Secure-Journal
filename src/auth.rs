use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::{PasswordHash, SaltString};
use argon2::{Argon2, PasswordHasher, PasswordVerifier};

use anyhow::{Ok, Result};
use colored::*;
use dialoguer::Input;
use rpassword::read_password;
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::Client;

pub async fn validate_creds(username: &str, password: &str) -> Result<()> {
    // usernmane check
    if username.trim().is_empty() {
        anyhow::bail!("usernmae cannot be empty..");
    }
    if username.len() < 3 && username.len() > 20 {
        anyhow::bail!("the len of the name is not valid..plz try again!");
    }
    if username.contains(' ') {
        anyhow::bail!("username cannot contain spaces");
    }
    
    // password check
    if password.len() < 6 {
        anyhow::bail!("password must be at least 6 chars long..");
    }
    if !password.chars().any(|c| c.is_uppercase()) {
        anyhow::bail!("password must include at least one uppercase char..");
    }
    if !password.chars().any(|c| c.is_lowercase()) {
        anyhow::bail!("password must include at least one lowercase char..");
    }
    if !password.chars().any(|c| c.is_ascii_digit()) {
        anyhow::bail!("password must include at least one number..");
    }
    if !password.chars().any(|c| "!@#$%^&*()-_=+[]{};:,.<>?".contains(c)) {
        anyhow::bail!("password must include at least one special character..");
    }
    
    Ok(())
}

use crate::models::{JournalEntry, User};

pub async fn signup_flow(db: &Surreal<Client>) -> Result<()> {
    //create new user
    let username = Input::<String>::new()
        .with_prompt("choose a username..")
        .interact()
        .map_err(|e| anyhow::anyhow!("input error: {e}"))?;
    println!("choose a password..");
    let password = read_password().unwrap();

    //check if user exists
    let check_query = format!("select * from user where username = {:?}", username);
    let mut check = db.query(check_query).await?;
    let existing: Vec<User> = check.take(0)?;
    if !existing.is_empty() {
        println!(
            "{}",
            "⚠️ Username already exists! Please choose another.".bright_red()
        );
        return Ok(());
    }

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
    //logins the user
    let username = Input::<String>::new()
        .with_prompt("username")
        .interact()
        .unwrap();

    println!("password:");
    let password = read_password().unwrap();

    let query = format!("select * from user where username = {:?}", username);
    let mut response = db.query(query).await?;
    let users: Vec<User> = response.take(0)?;

    //check if user exists
    if users.is_empty() {
        println!("{}", "no such user found..".bright_red());
        return Ok(None);
    }

    let user = &users[0];

    //parsing and hasing users creds
    let parsed_hash = PasswordHash::new(&user.password)?;
    if Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok()
    {
        println!(
            "{}",
            format!("login successful..! welcome, {}!", user.username).green()
        );
        Ok(Some(user.clone()))
    } else {
        println!("{}", "incorrect password..".red());
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
