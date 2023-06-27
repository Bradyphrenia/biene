// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod frontend;
mod communication;
mod commands;
mod database;
mod util;


// use crate::frontend::commands;
use crate::commands as other_commands;

fn app() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            // commands::test_db_connection,
            // commands::test_get,
            other_commands::connect_to_db,
        ])
        .plugin(tauri_plugin_store::Builder::default().build())
        .run(tauri::generate_context!())
        .expect("error while running tauri application")
}

fn main() {
    // Let's get started :)
    app();
}
