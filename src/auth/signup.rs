use anyhow::Result;
use colored::*;
use dialoguer::Input;
use indicatif::{ProgressBar, ProgressStyle};
use rpassword::read_password;

use argon2::{Argon2, PasswordHasher};
use argon2::password_hash::SaltString;
use argon2::password_hash::rand_core::OsRng;

use surrealdb::Surreal;
use surrealdb::engine::local::Db;

use std::time::Duration;

use crate::auth::validate::validate_creds;
use crate::models::models::{JournalEntry, User};

pub async fn signup_flow(db: &Surreal<Db>) -> Result<()> {
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
    spinner.enable_steady_tick(Duration::from_millis(50));
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
        std::thread::sleep(Duration::from_millis(30));
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
    let _: Vec<User> = db
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