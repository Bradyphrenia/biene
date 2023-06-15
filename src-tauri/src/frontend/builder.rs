// TODO: this file is outdated and should be removed in a future version
use super::commands;

pub fn app() {
    let tauri_app = tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![commands::test_db_connection, commands::test_get])
        .plugin(tauri_plugin_store::Builder::default().build());

    tauri_app
        .run(tauri::generate_context!())
        .expect("error while running tauri application")
}
