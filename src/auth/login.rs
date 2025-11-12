use anyhow::Result;
use colored::*;
use dialoguer::Input;
use indicatif::{ProgressBar, ProgressStyle};
use rpassword::read_password;
use std::time::Duration;
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::Client;

use crate::models::models::User;


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
    spinner.enable_steady_tick(Duration::from_millis(300));
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
    verify_spinner.enable_steady_tick(Duration::from_millis(200));
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