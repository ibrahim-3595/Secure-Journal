use colored::*;
use anyhow::{Ok, Result};

pub fn validate_creds(username: &str, password: &str) -> Result<()> {
    // usernmane check
    if username.trim().is_empty() {
        anyhow::bail!("{}", "usernmae cannot be empty..".yellow());
    }
    if username.len() < 3 || username.len() > 20 {
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
