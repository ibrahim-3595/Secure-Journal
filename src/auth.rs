use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::{PasswordHash, SaltString};
use argon2::{Argon2, PasswordHasher, PasswordVerifier};

use colored::*;
use dialoguer::Input;
use rpassword::read_password;
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::Client;

use crate::models::{User, JournalEntry};

pub async fn signup_flow(db: &Surreal<Client>) -> surrealdb::Result<()> {
    let username = Input::<String>::new()
        .with_prompt("choose a username..")
        .interact()
        .unwrap();
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
        .await?;
    println!("{}", "account created successfully..".green());

    Ok(())
}

pub async fn login_flow(db: &Surreal<Client>) -> surrealdb::Result<()> {
    let username = Input::<String>::new()
        .with_prompt("username")
        .interact()
        .unwrap();

    println!("password");
    let password = read_password().unwrap();

    let query = format!("select * from user where username ={:?}", username);
    let mut response = db.query(query).await?;
    let users: Option<User> = response.take(0)?;

    if let Some(user) = users {
        let parsed_hash = PasswordHash::new(&user.password).unwrap();
        if Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok()
        {
            println!("{:?}", "login successful! welcome, {username}!".bright_green());
        } else {
            println!("{}", "incorrect password...".red());
        }
    } else {
        println!("{}", "no such user found..".bright_red());
    }

    Ok(())
}

pub async fn list_users(db: &Surreal<Client>) -> surrealdb::Result<()> {
    let mut response = db.query("select * from user").await?;
    let users: Vec<User> = response.take(0)?;
    println!("{}", "registered user..".bright_green());
    for usr in users {
        println!("- {:?}", usr.username);
    }
    
    Ok(())
}

pub async fn new_entry(db: &Surreal<Client>, user: &User) -> surrealdb::Result<()> {
    let title = Input::<String>::new().with_prompt("Title").interact().unwrap();
    let content = Input::<String>::new().with_prompt("Content").interact().unwrap();
        
    let _: Option<JournalEntry> =db.create("entry").content(JournalEntry {
        id: None, 
        user: user.username.clone(), 
        title, 
        content, 
    }).await?;
    println!("{}", "journal entry has been saved..".green());
    
    Ok(())
}

pub async fn list_entries(db: Surreal<Client>, user: &User) -> surrealdb::Result<()> {
    let query = format!("select * from entry where user = {:?}", user.username);
    let mut resp = db.query(query).await?;
    let entries: Vec<JournalEntry> = resp.take(0)?;
    
    if entries.is_empty() {
        println!("{}", "no entires found for {user.username}".red());
    } else {
        println!("your journal entires: ");
        for (i, entry) in entries.iter().enumerate() {
            println!("{}. {} - {}", i+1, entry.title, entry.content);
        }
    }
    
    Ok(())
}
