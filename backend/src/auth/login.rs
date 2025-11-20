use sqlx::Row;
use colored::*;
use anyhow::Result;
use dialoguer::Input;
use crate::db::DbPool;
use serde::Deserialize;
use std::time::Duration;
use rpassword::read_password;
use crate::models::models::User;
use indicatif::{ProgressBar, ProgressStyle};
use argon2::{Argon2, PasswordHash, PasswordVerifier};

#[derive(Deserialize)]
pub struct AuthRequest {
    pub username: String,
    pub password: String,
}

pub async fn login_flow(db: &DbPool) -> Result<Option<User>> {
    let username = Input::<String>::new()
        .with_prompt("Username")
        .interact()?;

    println!("Password:");
    let password = read_password()?;

    if username.trim().is_empty() || password.is_empty() {
        println!("{}", "Username or Password cannot be EMPTY".bright_red());
        return Ok(None);
    }

    // Spinner
    let spinner = ProgressBar::new_spinner();
    spinner.set_message("Checking credentials...");
    spinner.enable_steady_tick(Duration::from_millis(100));
    spinner.set_style(
        ProgressStyle::default_spinner()
            .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏✔")
            .template("{spinner:.green} {msg}")?,
    );

    // Fetch user
    let user_row = sqlx::query("SELECT id, username, password_hash FROM users WHERE username = ?")
        .bind(&username)
        .fetch_optional(db)
        .await?;

    spinner.finish_and_clear();

    let Some(row) = user_row else {
        println!("{}", "No such user found".bright_red());
        return Ok(None);
    };

    let stored_hash: String = row.get("password_hash");
    let parsed_hash = PasswordHash::new(&stored_hash)?;

    // Password verifying spinner
    let verify_spinner = ProgressBar::new_spinner();
    verify_spinner.set_message("Verifying password...");
    verify_spinner.enable_steady_tick(Duration::from_millis(100));
    verify_spinner.set_style(
        ProgressStyle::default_spinner()
            .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏✔")
            .template("{spinner:.cyan} {msg}")?,
    );

    let valid = Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok();

    verify_spinner.finish_and_clear();

    if valid {
        println!(
            "{}",
            format!("Login Successful! Welcome, {}.", username).green()
        );

        // Construct User struct correctly
        Ok(Some(User {
            id: Some(row.get::<i64, _>("id")),  // i64 matches struct
            username,
            password: None, 
            password_hash: stored_hash,          // store hash, not plaintext
        }))
    } else {
        println!("{}", "Incorrect password..".red());
        Ok(None)
    }
}
