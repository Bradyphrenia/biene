// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod frontend;

fn main() {
    // connect to database
    // TODO: implement me

    // start tauri application
    frontend::builder::app()
}
