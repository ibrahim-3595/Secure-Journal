use crate::models::{AuthRequest, AuthResponse};

// const API_BASE_URL: &str = "http://127.0.0.1:8000/api";
const API_BASE_URL: &str = "http://127.0.0.1:8000/api";

pub async fn login(username: String, password: String) -> Result<AuthResponse, String> {
    let client = reqwest::Client::new();
    
    let response = client
        .post(format!("{}/login", API_BASE_URL))
        .json(&AuthRequest { username, password })
        .send()
        .await
        .map_err(|e| format!("Connection error: {}", e))?;

    response
        .json::<AuthResponse>()
        .await
        .map_err(|_| "Failed to parse response".to_string())
}

pub async fn signup(username: String, password: String) -> Result<AuthResponse, String> {
    let client = reqwest::Client::new();
    
    let response = client
        .post(format!("{}/signup", API_BASE_URL))
        .json(&AuthRequest { username, password })
        .send()
        .await
        .map_err(|e| format!("Connection error: {}", e))?;

    response
        .json::<AuthResponse>()
        .await
        .map_err(|_| "Failed to parse response".to_string())
}

// TODO: Add more API functions for journal entries
// pub async fn get_entries() -> Result<Vec<JournalEntry>, String> { ... }
// pub async fn create_entry(entry: JournalEntry) -> Result<AuthResponse, String> { ... }
// pub async fn update_entry(id: String, entry: JournalEntry) -> Result<AuthResponse, String> { ... }
// pub async fn delete_entry(id: String) -> Result<AuthResponse, String> { ... }