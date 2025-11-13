use argon2::{
    Argon2,
    password_hash::{PasswordHasher, SaltString},
};
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use std::fs::File;
use std::io::Write;
use rand::rngs::OsRng;

use crate::models::models::JournalEntry;

pub fn export_to_md(entries: &[JournalEntry], file_path: &str) -> anyhow::Result<()> {
    //progresbar
    let bar = ProgressBar::new(entries.len() as u64);
    bar.set_style(
        ProgressStyle::default_bar()
            .template(
                "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} entries",
            )
            .unwrap()
            .progress_chars("#>-"),
    );

    //hashing
    let mut rng = OsRng;
    let salt = SaltString::generate(&mut rng);
    let argon2 = Argon2::default();
    let hash = argon2.hash_password(file_path.as_bytes(), &salt)?;
    println!("Export hash: {}", hash.to_string().bright_black());


    //write file
    let mut file = File::create(file_path)?;
    for (i, entry) in entries.iter().enumerate() {
        writeln!(file, "# {}\n", entry.title)?;
        writeln!(file, "_created: {}_\n", entry.created_at)?;
        writeln!(file, "{}\n", entry.content)?;
        writeln!(file, "---\n")?;

        bar.inc(1);
        if i % 10 == 0 {
            bar.set_message(format!("Exported {}", entry.title).green().to_string());
        }
    }

    bar.finish_with_message("Export complete!");
    println!("{}", "✅ Journal exported successfully!".green());
    Ok(())
}
