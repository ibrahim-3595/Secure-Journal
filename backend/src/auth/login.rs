use anyhow::Result;
use colored::*;
use dialoguer::Input;
use indicatif::{ProgressBar, ProgressStyle};
use rpassword::read_password;
use std::time::Duration;
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use surrealdb::Surreal;
use surrealdb::engine::local::Db;

use crate::models::models::User;

pub async fn login_flow(db: &Surreal<Db>) -> Result<Option<User>> {
    //logins the user
    let username = Input::<String>::new()
        .with_prompt("Username")
        .interact()
        .unwrap();

    println!("Password:");
    let password = read_password().unwrap();

    // val_check
    if username.trim().is_empty() || password.is_empty() {
        println!("{}", "Username or Password cannot be EMPTY".bright_red());
        return Ok(None);
    }

    //spinner&progrrss-bar
    let spinner = ProgressBar::new_spinner();
    spinner.set_message("Checking credentials...");
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
        println!("{}", "No such user found".bright_red());
        return Ok(None);
    }

    let user = &users[0];

    //verify pass spinner
    let verify_spinner = ProgressBar::new_spinner();
    verify_spinner.set_message("Verifying password...");
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
            format!("Login Successful! Welcome, {}.", user.username).green()
        );
        Ok(Some(user.clone()))
    } else {
        println!("{}", "Incorrect password..".red());
        Ok(None)
    }
}

pub async fn login_api(
    db: &Surreal<Db>,
    username: &str,
    password: &str,
) -> Result<Option<User>> {

    let query = format!("select * from user where username = {:?}", username);
    let mut response = db.query(query).await?;
    let users: Vec<User> = response.take(0)?;

    if users.is_empty() {
        return Ok(None);
    }

    let user = &users[0];

    let parsed_hash = PasswordHash::new(&user.password)?;
    let valid = Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok();

    if valid {
        Ok(Some(user.clone()))
    } else {
        Ok(None)
    }
}