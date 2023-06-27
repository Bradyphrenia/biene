use std::path::PathBuf;

use tauri::Wry;
use tauri_plugin_store::{with_store, JsonValue, StoreCollection};

use super::structs::Database;


fn get_from_store(
    app_handle: tauri::AppHandle,
    stores: tauri::State<'_, StoreCollection<Wry>>,
    store_key: &str,
) -> Result<std::option::Option<JsonValue>, tauri_plugin_store::Error> {
    let path = PathBuf::from(".settings.dat");
    with_store(app_handle, stores, path, |store| {
        Ok(store.get(store_key).cloned())
    })
}

pub fn get_db_from_store(
    app_handle: tauri::AppHandle,
    stores: tauri::State<'_, StoreCollection<Wry>>,
) -> Database {
    let mut db_settings: Database = Default::default();
    match get_from_store(app_handle, stores, "database") {
        Ok(res) => {
            db_settings = serde_json::from_value(res.clone().into()).unwrap();
        }
        Err(err) => {
            println!("{}", err);
        }
    }
    db_settings
}
