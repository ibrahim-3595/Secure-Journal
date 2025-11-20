use anyhow::{Ok, Result};
use colored::*;
use dialoguer::Input;
use surrealdb::Surreal;
use chrono::Utc;
use surrealdb::engine::local::Db;
use crate::models::models::{JournalEntry, User};

pub async fn new_entry(db: &Surreal<Db>, user: &User) -> Result<()> {
    let title = Input::<String>::new()
        .with_prompt("Title")
        .interact()
        .unwrap();
    let content = Input::<String>::new()
        .with_prompt("Content")
        .interact()
        .unwrap();
    let tags_input = Input::<String>::new()
        .with_prompt("Tags (comma seperated)")
        .allow_empty(true)
        .interact()
        .unwrap();
    let now = Utc::now().to_rfc3339();
    let tags: Vec<String> = tags_input
        .split(',')
        .map(|t| t.trim().to_string())
        .filter(|t| !t.is_empty())
        .collect();

    let _: Vec<JournalEntry> = db
        .create("entry")
        .content(JournalEntry {
            id: None,
            user: user.username.clone(),
            title,
            content,
            tags,
            created_at: now.clone(),
            updated_at: now,
        })
        .await?;

    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    println!("{}", "Journal entry has been saved!".green());

    Ok(())
}

pub async fn delete_entry(db: &Surreal<Db>, user: &User) -> Result<()> {
    let query = format!("select * from entry where user = {:?}", user.username);
    let mut resp = db.query(query).await?;
    let entries: Vec<JournalEntry> = resp.take(0)?;

    if entries.is_empty() {
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        println!("{}", "No entries to delete".red());
        return Ok(());
    }

    println!("Your journal entries: ");
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    for (i, entry) in entries.iter().enumerate() {
        println!("{}. {} - {}", i + 1, entry.title, entry.content);
    }

    let index = Input::<usize>::new()
        .with_prompt("Enter the number of entires to delete: ")
        .interact()
        .unwrap();
    if index == 0 || index > entries.len() {
        println!("{}", "Invalid entry number".red());
        return Ok(());
    }

    let entry_to_delete = &entries[index - 1];
    if let Some(id) = &entry_to_delete.id {
        let delete_query = format!("delete {}", id);
        db.query(delete_query).await?;
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        println!("{}", "Journal entry deleted successfully!".green());
    } else {
        println!("{}", "Error: Entry has no valid ID.".red());
    }

    Ok(())
}

pub async fn list_users(db: &Surreal<Db>) -> Result<()> {
    let mut response = db.query("select * from user").await?;
    let users: Vec<User> = response.take(0)?;
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    println!("{}", "Registered users: ".bright_green());
    for usr in users {
        println!("- {:?}", usr.username);
    }

    Ok(())
}

pub async fn list_entries(db: &Surreal<Db>, user: &User) -> Result<()> {
    let query = format!("select * from entry where user = {:?}", user.username);
    let mut resp = db.query(query).await?;
    let entries: Vec<JournalEntry> = resp.take(0)?;

    if entries.is_empty() {
        println!(
            "{}",
            format!("No entires found for {}", user.username).red()
        );
    } else {
        println!("Your journal entires: ");
        tokio::time::sleep(std::time::Duration::from_secs(2)).await;
        for (i, entry) in entries.iter().enumerate() {
            println!(
                "\n{}. {} - {}\n   created at: {}\n    tags: {}\n",
                i + 1,
                entry.title,
                entry.content,
                entry.created_at,
                if entry.tags.is_empty() {
                    "(none)".to_string()
                } else {
                    entry.tags.join(", ")
                }
            );
        }
    }

    Ok(())
}

pub async fn update_entry(db: &Surreal<Db>, user: &User) -> Result<()> {
    let query = format!("select * from entry where user = {:?}", user.username);
    let mut resp = db.query(query).await?;
    let entries: Vec<JournalEntry> = resp.take(0)?;

    if entries.is_empty() {
        println!("{}", "no entries to update..".red());
        return Ok(());
    }

    println!("your journal entries: ");
    for (i, entry) in entries.iter().enumerate() {
        println!("{}. {} - {}", i + 1, entry.title, entry.content);
    }

    let index = Input::<usize>::new()
        .with_prompt("enter the num of entries you wanna update: ")
        .interact()
        .unwrap();
    if index == 0 || index > entries.len() {
        println!("{}", "invalid entry number..".red());
        return Ok(());
    }

    let entry = &entries[index - 1];
    let new_content = Input::<String>::new()
        .with_prompt("new content")
        .interact()
        .unwrap();
    let new_tags = Input::<String>::new()
        .with_prompt("new tags( comma seperated)")
        .allow_empty(true)
        .interact()
        .unwrap();
    let tags: Vec<String> = new_tags
        .split(',')
        .map(|t| t.trim().to_string())
        .filter(|t| !t.is_empty())
        .collect();

    let updated_at = Utc::now().to_rfc3339();

    let updated_query = format!(
        "updated {} set content: {:?}, tags: {:?}, updated_at: {:?}",
        entry.id.as_ref().unwrap(),
        new_content,
        tags,
        updated_at
    );

    db.query(updated_query).await?;
    println!("{}", "entry updated successfully..".bright_green());

    Ok(())
}

//
pub async fn get_entries_for_user(db: &Surreal<Db>, user: &User) -> Result<Vec<JournalEntry>> {
    let sql = format!(
        "SELECT * FROM journal_entries WHERE user = '{}';",
        user.username
    );
    let mut response = db.query(sql).await?;
    let entries: Vec<JournalEntry> = response.take(0)?;
    Ok(entries)
}
