// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use log::{info, LevelFilter};
use simple_logger::SimpleLogger;

mod app;
mod commands;
mod communication;
mod database;
mod diesel;
mod frontend;
mod util;

use app::init_app;

fn main() {
    SimpleLogger::new()
        .with_level(LevelFilter::Debug)
        .env()
        .init()
        .unwrap();
    info!("Starting biene");
    let app = init_app();
    app.run(|_, _| {})
}
