use database::cli::input_date;
use database::dbase::{db_execute, init_db};

// use crate::cli::cli::input_date;

const HIVE_NAMES: [&str; 8] = [
    "Volk 01", "Volk 02", "Volk 03", "Volk 04", "Volk 05", "Volk 06", "Volk 07", "Volk 08",
];

fn create_sql(date: String, hive_name: String) -> String {
    format!("INSERT INTO behandlung (datum, volk, koenigin, stifte, offene, verdeckelte, weiselzelle, spielnaepfe, sanftmut, volksstaerke, anz_brutwaben, memo, krankheit, behandlung, anwendungsform, menge, mengeneinheit, konzentration, wartezeit, zulassungsnummer, meldepflicht) VALUES ('{}', '{}', NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, 'VarroMed', 'geträufelt', 10, 'ml', NULL, NULL, NULL, NULL);", date, hive_name)
}

fn process_hive(date: String, hive: String) {
    let sql = create_sql(date, hive);
    println!("{}", sql);
    let exec_result = db_execute(sql.as_str(), init_db());
    if exec_result == 1 {
        println!("Done.");
    } else {
        println!("Error.");
    }
}

pub fn main() {
    let date = input_date("Datum der Behandlung:");
    for hive in HIVE_NAMES {
        process_hive(date.to_string(), hive.to_string());
    }
}
