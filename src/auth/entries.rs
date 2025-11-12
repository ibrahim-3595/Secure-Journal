use anyhow::Result;
use colored::*;
use dialoguer::Input;
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::Client;
use crate::models::{JournalEntry, User};

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

pub async fn list_users(db: &Surreal<Client>) -> Result<()> {
    let mut response = db.query("select * from user").await?;
    let users: Vec<User> = response.take(0)?;
    println!("{}", "registered user..".bright_green());
    for usr in users {
        println!("- {:?}", usr.username);
    }

    Ok(())
}

pub async fn list_entries(db: &Surreal<Client>, user: &User) -> Result<()> {
    let query = format!("select * from entry where user = {:?}", user.username);
    let mut resp = db.query(query).await?;
    let entries: Vec<JournalEntry> = resp.take(0)?;

    if entries.is_empty() {
        println!("{}", format!("no entires found for {}", user.username).red());
    } else {
        println!("your journal entires: ");
        for (i, entry) in entries.iter().enumerate() {
            println!("{}. {} - {}", i + 1, entry.title, entry.content);
        }
    }

    Ok(())
}

