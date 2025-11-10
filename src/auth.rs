use argon2::{Argon2, PasswordHasher, PasswordVerifier};
use argon2::password_hash::{SaltString, PasswordHash};
use argon2::password_hash::rand_core::OsRng;

use rpassword::read_password;
use dialoguer::Input;
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::Client;

// use rand::rngs::OsRng;

use crate::models::User;

pub async fn signup_flow(db: &Surreal<Client>) -> surrealdb::Result<()> {
    let username = Input::<String>::new()
        .with_prompt("choose a username..")
        .interact()
        .unwrap();
    println!("choose a password..");
    let password = read_password().unwrap();

    //hashing..
    let mut rng = OsRng;
    let salt = SaltString::generate(&mut rng);
    let hashed = Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .unwrap()
        .to_string();

    //insert
    db.create("user")
        .content(User {
            username,
            password: hashed,
            id: None,
        })
        .await?;
    println!("account created successfully..");

    Ok(())
}

pub async fn login_flow(db: &Surreal<Client>) -> surrealdb::Result<()> {
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
            let parsed_hash = PasswordHash::new(&user.password).unwrap();
            if Argon2::default()
                .verify_password(password.as_bytes(), &parsed_hash)
                .is_ok()
            {
                println!("login succesful! welcome, {}!", username);
            } else {
                println!("incorrect password...");
            }
        } else {
            println!("no such user found..");
        }
    } else {
        println!("no data fetched from db..");
    }

    Ok(())
}

pub async fn list_users(db: &Surreal<Client>) -> surrealdb::Result<()> {
    let mut response = db.query("select username from user").await?;
    let users: Vec<User> = response.take(0)?;
    println!("registered user..");
    for usr in users {
        println!("- {:?}", usr.username);
    }
    Ok(())
}
