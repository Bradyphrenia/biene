mod cli;
mod database;

use crate::cli::cli::input_date;
use crate::database::database::db_execute;
use crate::database::database::init_db;
use postgres::Client;

const HIVE_NAMES: [&str; 8] = [
    "Volk 01", "Volk 02", "Volk 03", "Volk 04", "Volk 05", "Volk 06", "Volk 07", "Volk 08",
];
fn create_sql(date: &str, hive: &str) -> String {
    return format!( "INSERT INTO behandlung (datum, volk, koenigin, stifte, offene, verdeckelte, weiselzelle, spielnaepfe, sanftmut, volksstaerke, anz_brutwaben, memo, krankheit, behandlung, anwendungsform, menge, mengeneinheit, konzentration, wartezeit, zulassungsnummer, meldepflicht) VALUES ('{}', '{}', NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, 'VarroMed', 'geträufelt', 10, 'ml', NULL, NULL, NULL, NULL);", date, hive);
}

fn process_hive(hive: &str, date: String, db: Client) {
    let sql = create_sql(&date, hive);
    println!("{}", &sql);
    let exec_result = db_execute(sql.as_str(), db);
    if exec_result == 1 {
        println!("Done.");
    } else {
        println!("Error.");
    }
}

pub fn main() {
    let date = input_date("Datum der Behandlung:");
    for &hive in &HIVE_NAMES {
        process_hive(hive, date.to_string(), init_db());
    }
}
