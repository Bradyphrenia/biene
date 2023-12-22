mod cli;
mod database;

use crate::cli::cli::input_date;
use crate::database::database::db_execute;
use crate::database::database::init_db;

pub fn main() {
    let folks = vec![
        "Volk 01", "Volk 02", "Volk 03", "Volk 05", "Volk 06", "Volk 08", "Volk 09", "Volk 11",
    ];
    let sql_1 = "INSERT INTO behandlung (datum, volk, koenigin, stifte, offene, verdeckelte, weiselzelle, spielnaepfe, sanftmut, volksstaerke, anz_brutwaben, memo, krankheit, behandlung, anwendungsform, menge, mengeneinheit, konzentration, wartezeit, zulassungsnummer, meldepflicht) VALUES ('";
    let sql_2 = "', '";
    let sql_3 = "', NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, 'VarroMed', 'geträufelt', 10, 'ml', NULL, NULL, NULL, NULL);";
    let date = input_date("Datum der Behandlung: ");
    for item in &folks {
        let volk = item;
        let sql = format!("{}{}{}{}{}", sql_1, date, sql_2, volk, sql_3);
        println!("{}", &sql);
        let test = db_execute(sql.as_str(), init_db());
        if test == 1 {
            println!("Done.");
        } else {
            println!("Error.");
        }
    }
}
