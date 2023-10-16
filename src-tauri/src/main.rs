// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{App, CustomMenuItem, Menu, MenuItem, Submenu, Manager, WindowBuilder, AboutMetadata};

mod commands;
mod communication;
mod database;
mod frontend;
mod util;
mod diesel;

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
            .add_native_item(MenuItem::CloseWindow)
            .add_native_item(MenuItem::Quit)
    );
    Menu::new()
        .add_submenu(submenu)
}

fn app() -> App {
    let app = tauri::Builder::default()
        .menu(init_menu())
        .setup(|app| {
            let main_window = app.get_window("main").unwrap();
            let handle = app.handle();
            let window = main_window.clone();

            main_window.on_menu_event(move |event| {
                match event.menu_item_id() {
                    "settings" => {
                        WindowBuilder::new(
                            &handle,
                            "settings",
                            tauri::WindowUrl::App("#/settings".into())
                        )
                            .title("settings")
                            .build()
                            .unwrap();
                    }
                    _ => {}
                }
            });
            Ok(())

        })
        .invoke_handler(tauri::generate_handler![commands::connect_to_db, commands::test,])
        .plugin(tauri_plugin_store::Builder::default().build())
        .build(tauri::generate_context!())
        .expect("error while running tauri application");
        // .run(tauri::generate_context!())
    app
}

fn main() {
    // Let's get started :)
    let tt = app();
    tt.run(|_, _| {})
}
