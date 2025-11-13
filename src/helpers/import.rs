use anyhow::Result;
use argon2::{
    Argon2,
    password_hash::{PasswordHasher, PasswordVerifier, SaltString},
};
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use std::fs;

use crate::models::models::JournalEntry;

pub fn import_md(file_path: &str) -> Result<Vec<JournalEntry>> {
    let content = fs::read_to_string(file_path)?;

    //bar
    let bar = ProgressBar::new_spinner();
    bar.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.green} {msg}")
            .unwrap(),
    );
    bar.set_message("validating file integrity");

    //integrity check
    let argon2 = Argon2::default();
    let fake_salt = SaltString::generate(&mut rand::rngs::OsRng);
    // just to prove this file can be hashed (not real security yet)
    let _ = argon2.hash_password(file_path.as_bytes(), &fake_salt)?;

    //parse
    let mut entries = Vec::new();
    let mut current = JournalEntry::default();
    for line in content.lines() {
        if line.starts_with("# ") {
            if !current.title.is_empty() {
                entries.push(current);
                current = JournalEntry::default();
            }
            current.title = line.trim_start_matches("# ").to_string();
        } else if line.starts_with("_created:") {
            current.created_at = line.trim_start_matches("_created:").trim().to_string();
        } else if line == "---" {
            // end of entry
        } else {
            current.content.push_str(line);
            current.content.push('\n');
        }
    }
    if !current.title.is_empty() {
        entries.push(current);
    }

    bar.finish_with_message(
        format!("Imported {} entries!", entries.len())
            .green()
            .to_string(),
    );
    Ok(entries)
}
