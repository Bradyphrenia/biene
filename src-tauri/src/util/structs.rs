use serde::{Deserialize, Serialize};

// Response codes for Tauri requests from backend to frontend
#[derive(Serialize, Deserialize)]
pub enum TauriCode {
    Ok = 200,
    DBConnectionError = 400,
}

#[derive(Serialize, Deserialize)]
pub struct TauriResponse {
    pub code: TauriCode,
    pub msg: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Database {
    pub url: String,
    pub port: String,
    pub database: String,
    pub user: String,
    pub password: String,
}

impl Database {
    fn new() -> Self {
        Default::default()
    }
}

impl Default for Database {
    fn default() -> Self {
        Database {
            url: String::from("127.0.0.1"),
            port: String::from("5432"),
            database: String::from("biene"),
            user: String::from("postgres"),
            password: String::from("postgres"),
        }
    }
}
