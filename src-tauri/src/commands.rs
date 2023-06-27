use diesel::pg::PgConnection;
use diesel::prelude::*;
use tauri::{self, Wry};
use tauri_plugin_store::StoreCollection;

use crate::util::structs::{TauriResponse, TauriCode};
use crate::util::store::{get_db_from_store};


#[tauri::command]
pub fn connect_to_db (
    app_handle: tauri::AppHandle,
    store: tauri::State<'_, StoreCollection<Wry>>
) -> TauriResponse {

    let db_settings = get_db_from_store(app_handle, store);

    // TODO: add tauri store content
    let connection = PgConnection::establish(&format!(
        "postgres://{}:{}@{}:{}/{}",
        db_settings.user,
        db_settings.password,
        db_settings.url,
        db_settings.port,
        db_settings.database,
    ));

    // return message and code to frontend
    match connection {
        Ok(_) => TauriResponse {
            code: TauriCode::Ok,
            msg: String::from("Connection established"),
        },
        Err(e) => TauriResponse {
            code: TauriCode::DBConnectionError,
            msg: String::from(format!("Connection to database failed: {:?}", e)),
        },
    }
}

#[tauri::command]
pub fn get_from_db () {
    todo!();
}

#[tauri::command]
pub fn add_to_db () {
    todo!();
}

#[tauri::command]
pub fn update_db_entry () {
    todo!();
}
