// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{CustomMenuItem, Menu, MenuItem, Submenu, Manager, WindowBuilder, AboutMetadata};

mod commands;
mod communication;
mod database;
mod frontend;
mod util;

fn init_menu() -> Menu {
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let close = CustomMenuItem::new("close".to_string(), "Close");
    let settings = CustomMenuItem::new("settings".to_string(), "Settings...");
    let submenu = Submenu::new(
        "Bees",
        Menu::new()
            .add_native_item(MenuItem::About("Bees".to_string(), AboutMetadata::new()))
            .add_item(quit)
            .add_item(settings)
            .add_native_item(MenuItem::Quit)
    );
    Menu::new()
        // .add_item(CustomMenuItem::new("hide", "Hide"))
        .add_submenu(submenu)
}

fn app() {
    tauri::Builder::default()
        .menu(init_menu())
        .invoke_handler(tauri::generate_handler![commands::connect_to_db, commands::test,])
        .plugin(tauri_plugin_store::Builder::default().build())
        .run(tauri::generate_context!())
        .expect("error while running tauri application")
}

fn main() {
    // Let's get started :)
    app();
}
