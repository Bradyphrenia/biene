use diesel::pg::PgConnection;
use diesel::prelude::*;
use tauri::{self, Wry};
use tauri_plugin_store::StoreCollection;

use crate::util::store::get_db_from_store;
use crate::util::structs::{Database, TauriCode, TauriResponse};

use crate::diesel::models::{self, *};
use crate::diesel::schema::durchsicht::dsl::*;

use diesel::prelude::*;

#[tauri::command]
pub fn connect_to_db(
    app_handle: tauri::AppHandle,
    store: tauri::State<'_, StoreCollection<Wry>>,
) -> TauriResponse {
    let Database {
        user,
        password,
        url,
        port,
        database,
    } = get_db_from_store(app_handle, store);

    // TODO: add tauri store content
    let connection = PgConnection::establish(&format!(
        "postgres://{}:{}@{}:{}/{}",
        user, password, url, port, database,
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

/// Reading durchsicht table from database and providing results to frontend
#[tauri::command]
pub fn request_review(
    app_handle: tauri::AppHandle,
    store: tauri::State<'_, StoreCollection<Wry>>,
) -> Result<Vec<models::Durchsicht>, String> {
    let connection = &mut connect(app_handle, store);

    let results: Vec<_> = durchsicht
        .select(Durchsicht::as_select())
        .load(connection)
        .expect("Error loading durchsicht");

    // TODO: Return Object with status number
    Ok(results)
}

#[tauri::command]
pub fn get_from_db() {
    todo!();
}

#[tauri::command]
pub fn add_to_db() {
    todo!();
}

#[tauri::command]
pub fn update_db_entry() {
    todo!();
}
