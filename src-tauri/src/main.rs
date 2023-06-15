// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// mod frontend;
mod database;

use crate::frontend::commands;

fn app() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            commands::test_db_connection,
            commands::test_get
        ])
        .plugin(tauri_plugin_store::Builder::default().build())
        .run(tauri::generate_context!())
        .expect("error while running tauri application")
}

fn main() {
    // start tauri application
    frontend::builder::app();
}
