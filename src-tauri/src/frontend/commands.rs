use serde::{Serialize, Deserialize};

#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}


#[derive(Debug, Serialize, Deserialize)]
struct Database {
    url: String,
    port: String,
    database: String,
    user: String,
    password: String,
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
            password: String::from("postgres")
        }
    }
}


#[tauri::command]
pub fn connect_db() {
    // TODO: implement me
}

#[tauri::command]
pub fn disconnect_db() {
    // TODO: implement me
}

#[tauri::command]
pub fn get_table_list(table_name: &str) {
    // TODO: implement me
}
