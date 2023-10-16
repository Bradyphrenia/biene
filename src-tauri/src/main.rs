// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{AboutMetadata, App, CustomMenuItem, Manager, Menu, MenuItem, Submenu, WindowBuilder};

mod commands;
mod communication;
mod database;
mod diesel;
mod frontend;
mod util;

fn init_menu() -> Menu {
    let settings = CustomMenuItem::new("settings".to_string(), "Settings...");
    let submenu = Submenu::new(
        "Bees",
        Menu::new()
            .add_native_item(MenuItem::About("Bees".to_string(), AboutMetadata::new()))
            .add_item(settings)
            .add_native_item(MenuItem::CloseWindow)
            .add_native_item(MenuItem::Quit),
    );
    Menu::new().add_submenu(submenu)
}

fn init_app() -> App {
    tauri::Builder::default()
        .menu(init_menu())
        .setup(|app| {
            let main_window = app.get_window("main").unwrap();
            let handle = app.handle();

            main_window.on_menu_event(move |event| match event.menu_item_id() {
                "settings" => {
                    WindowBuilder::new(
                        &handle,
                        "settings",
                        tauri::WindowUrl::App("#/settings".into()),
                    )
                    .title("settings")
                    .build()
                    .unwrap();
                }
                _ => {}
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::connect_to_db,
            commands::test,
        ])
        .plugin(tauri_plugin_store::Builder::default().build())
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
}

fn main() {
    let app = init_app();
    app.run(|_, _| {})
}
