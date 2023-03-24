use tauri_plugin_store::{StoreCollection, with_store};
use serde_json::json;
use tauri::{Manager, Wry};
use std::path::PathBuf;

use super::commands;

pub fn app () {

    tauri::Builder::default()

        .invoke_handler(tauri::generate_handler![commands::greet])

        .plugin(tauri_plugin_store::Builder::default().build())
        .setup(|app2| {
            let app_handle = app2.handle().clone();
            let stores = app2.state::<StoreCollection<Wry>>();
            let path = PathBuf::from(".settings.dat");
            let result = with_store(app_handle, stores, path, |store| store.insert("a".to_string(), json!({ "code": 200 })));
                    result.map_err(|error| Box::new(error) as Box<dyn std::error::Error>)
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application")
}
