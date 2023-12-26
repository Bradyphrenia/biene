// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod commands;
mod communication;
mod database;
mod diesel;
mod frontend;
mod util;

use app::init_app;

fn main() {
    let app = init_app();
    app.run(|_, _| {})
}
