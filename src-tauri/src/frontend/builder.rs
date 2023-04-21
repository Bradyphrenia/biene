use super::commands;

pub fn app() {
    let tauri_app = tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![commands::test_db_connection])
        .plugin(tauri_plugin_store::Builder::default().build());

    tauri_app
        .run(tauri::generate_context!())
        .expect("error while running tauri application")
}
