use serde::{Deserialize, Serialize};

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
