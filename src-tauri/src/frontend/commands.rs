use tauri_plugin_store::{JsonValue, StoreCollection, with_store};
use tauri::Wry;
use std::path::PathBuf;
use postgres::{Client, NoTls};

#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}
use super::structs::Database;


fn get_from_store(
    app_handle: tauri::AppHandle,
    stores: tauri::State<'_, StoreCollection<Wry>>,
    store_key: &str
) -> Result<std::option::Option<JsonValue>, tauri_plugin_store::Error> {
    let path = PathBuf::from(".settings.dat");
    with_store(app_handle, stores, path, |store| Ok(store.get(store_key).cloned()))
}

fn get_db_from_store(
    app_handle: tauri::AppHandle,
    stores: tauri::State<'_, StoreCollection<Wry>>
) -> Database {
    let mut db_settings: Database = Default::default();
    match get_from_store(app_handle, stores, "database") {
        Ok(res) => { db_settings = serde_json::from_value(res.clone().into()).unwrap(); },
        Err(err) => { println!("{}", err); }
    }
    db_settings
}

#[tauri::command]
pub fn connect_db(
    app_handle: tauri::AppHandle,
    stores: tauri::State<'_, StoreCollection<Wry>>
) {
    let db_settings = get_db_from_store(app_handle, stores);
    // get db settings with db_settings.url ...
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
