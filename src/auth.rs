use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::{PasswordHash, SaltString};
use argon2::{Argon2, PasswordHasher, PasswordVerifier};

// use crate::common::error::{AppError, Result};

use anyhow::{Ok, Result};
use colored::*;
use dialoguer::{Confirm, Input};
use indicatif::{ProgressBar, ProgressStyle};
use rpassword::read_password;
use std::time::Duration;
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::Client;

use crate::models::{JournalEntry, User};

fn validate_creds(username: &str, password: &str) -> Result<()> {
    // usernmane check
    if username.trim().is_empty() {
        anyhow::bail!("{}", "usernmae cannot be empty..".yellow());
    }
    if username.len() < 3 && username.len() > 20 {
        anyhow::bail!(
            "{}",
            "the len of the name is not valid..plz try again!".yellow()
        );
    }
    if username.contains(' ') {
        anyhow::bail!("{}", "username cannot contain spaces".yellow());
    }

    // password check
    if password.len() < 6 {
        anyhow::bail!("{}", "password must be at least 6 chars long..".yellow());
    }
    if !password.chars().any(|c| c.is_uppercase()) {
        anyhow::bail!(
            "{}",
            "password must include at least one uppercase char..".yellow()
        );
    }
    if !password.chars().any(|c| c.is_lowercase()) {
        anyhow::bail!(
            "{}",
            "password must include at least one lowercase char..".yellow()
        );
    }
    if !password.chars().any(|c| c.is_ascii_digit()) {
        anyhow::bail!("{}", "password must include at least one number..".yellow());
    }
    if !password
        .chars()
        .any(|c| "!@#$%^&*()-_=+[]{};:,.<>?".contains(c))
    {
        anyhow::bail!(
            "{}",
            "password must include at least one special character..".yellow()
        );
    }

    Ok(())
}

pub async fn signup_flow(db: &Surreal<Client>) -> Result<()> {
    //create new user
    let username = Input::<String>::new()
        .with_prompt("choose a username")
        .interact()
        .map_err(|e| anyhow::anyhow!("input error: {e}"))?;
    println!("choose a password");
    let password = read_password().unwrap();

    //val_creds
    if let Err(e) = validate_creds(&username, &password) {
        println!("{}", format!("{}", e).bright_red());
        return Ok(());
    }

    //spinners&progress-bars
    let spinner = ProgressBar::new_spinner();
    spinner.set_message("checking if username is available..");
    spinner.enable_steady_tick(Duration::from_millis(100));
    spinner.set_style(
        ProgressStyle::default_spinner()
            .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏✔")
            .template("{spinner:.green} {msg}")
            .unwrap(),
    );

    //check if user exists
    let check_query = format!("select * from user where username = {:?}", username);
    let mut check = db.query(check_query).await?;
    let existing: Vec<User> = check.take(0)?;
    spinner.finish_and_clear();
    if !existing.is_empty() {
        println!(
            "{}",
            "username already exists..please choose another one!!".bright_red()
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

    //hashing progress..
    let bar = ProgressBar::new(100);
    bar.set_style(
        ProgressStyle::with_template(
            "{spinner:.blue} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}% - {msg}",
        )
        .unwrap()
        .progress_chars("=>-"),
    );
    bar.set_message("hasing password securely..");
    for i in 0..100 {
        bar.set_position(i);
        std::thread::sleep(Duration::from_millis(10));
    }

    //hashing..
    let mut rng = OsRng;
    let salt = SaltString::generate(&mut rng);
    let hashed = Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .unwrap()
        .to_string();
    bar.finish_with_message("password hashed successfully..");

    //insert..
    let spinner = ProgressBar::new_spinner();
    spinner.set_message("created your acc..");
    spinner.enable_steady_tick(Duration::from_millis(100));
    spinner.set_style(
        ProgressStyle::default_spinner()
            .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏✔")
            .template("{spinner:.green} {msg}")
            .unwrap(),
    );
    let _: Option<User> = db
        .create("user")
        .content(User {
            username,
            password: hashed,
            id: None,
        })
        .await
        .map_err(|e| anyhow::anyhow!("databse error: {e}"))?;
    spinner.finish_and_clear();
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

    // val_check
    if username.trim().is_empty() || password.is_empty() {
        println!("{}", "username or password cannot be empty...".bright_red());
        return Ok(None);
    }

    //spinner&progrrss-bar
    let spinner = ProgressBar::new_spinner();
    spinner.set_message("checking credentials...");
    spinner.enable_steady_tick(Duration::from_millis(100));
    spinner.set_style(
        ProgressStyle::default_spinner()
            .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏✔")
            .template("{spinner:.green} {msg}")
            .unwrap(),
    );

    let query = format!("select * from user where username = {:?}", username);
    let mut response = db.query(query).await?;
    let users: Vec<User> = response.take(0)?;
    spinner.finish_and_clear();

    //check if user exists
    if users.is_empty() {
        println!("{}", "no such user found..".bright_red());
        return Ok(None);
    }

    let user = &users[0];

    //verify pass spinner
    let verify_spinner = ProgressBar::new_spinner();
    verify_spinner.set_message("verifying password...");
    verify_spinner.enable_steady_tick(Duration::from_millis(100));
    verify_spinner.set_style(
        ProgressStyle::default_spinner()
            .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏✔")
            .template("{spinner:.cyan} {msg}")
            .unwrap(),
    );

    //parsing and hasing users creds
    let parsed_hash = PasswordHash::new(&user.password)?;
    let valid = Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok();
    verify_spinner.finish_and_clear();
    if valid {
        println!(
            "{}",
            format!("login successful! welcome, {}.", user.username).green()
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
pub async fn delete_entry(db: &Surreal<Client>, user: &User) -> Result<()> {
    let query = format!("select * from entry where user = {:?}", user.username);
    let mut resp = db.query(query).await?;
    let entries: Vec<JournalEntry> = resp.take(0)?;

    if entries.is_empty() {
        println!("{}", "no entries to delete..".red());
        return Ok(());
    }

    println!("your journal entries..");
    for (i, entry) in entries.iter().enumerate() {
        println!("{}. {} - {}", i + 1, entry.title, entry.content);
    }

    let index = Input::<usize>::new()
        .with_prompt("enter the num of entires to delete..")
        .interact()
        .unwrap();
    if index == 0 || index > entries.len() {
        println!("{}", "invalid entry number..".red());
        return Ok(());
    }

    let entry_to_delete = &entries[index - 1];
    if let Some(id) = &entry_to_delete.id {
        let delete_query = format!("delete {}", id);
        db.query(delete_query).await?;
        println!("{}", "journal entry deleted successfully..".green());
    } else {
        println!("{}", "err: entry has no valid ID.".red());
    }

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
