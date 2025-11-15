#[derive(Clone, Debug, PartialEq)]
pub struct AppState {
    pub logged_in: bool,
    pub username: String,
    pub token: String,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            logged_in: false,
            username: String::new(),
            token: String::new(),
        }
    }
}