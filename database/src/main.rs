mod cli;
mod database;

use crate::cli::cli::{input_bool, input_date, input_number, input_string};
use crate::database::dbase::Durchsicht;
use database::dbase::{
    db_execute, durchsicht_fetchall, durchsicht_fetchone, init_db, volk_fetchall, volk_fetchone,
};

// console app
fn main() {
    loop {
        let mut ds: Durchsicht = Default::default();
        let input = input_date("Datum?  JJJJ-MM-TT");
        ds.datum = input;
        let input = input_string("Volk?  Volk 99");
        ds.volk = input;
        let input = input_bool("Königin?  1 | 0");
        ds.koenigin = input;
        let input = input_bool("Stifte?  1 | 0");
        ds.stifte = input;
        let input = input_bool("offene Brut?  1 | 0");
        ds.offene = input;
        let input = input_bool("verdeckelte Brut?  1 | 0");
        ds.verdeckelte = input;
        let input = input_bool("Weiselzelle?  1 | 0");
        ds.weiselzelle = input;
        let input = input_bool("Spielnäpfe?  1 | 0");
        ds.spielnaepfe = input;
        let input = input_number("Sanftmut?  1 - 5", 1, 5);
        ds.sanftmut = input;
        let input = input_number("Volkstärke?  1 - 5", 1, 5);
        ds.volksstaerke = input;
        let input = input_number("Anzahl Brutwaben?  99", 0, 24);
        ds.anz_brutwaben = input;
        let input = input_string("Bemerkungen?");
        ds.memo = input;
        let sql = Durchsicht::ds_to_sql(&ds);
        println!("SQL-Script:");
        println!("{}", &sql);
        let db = init_db();
        let lines = db_execute(sql.as_str(), db);
        println!("Es wurde(n) {} Zeile(n) hinzugefügt.", lines);
        println!();
        let input = input_bool("Weitere Durchsichten?  1 | 0");
        if input == false {
            break;
        }
    }
}
