mod database;

use crate::database::dbase::{Durchsicht};
use database::dbase::{
    db_execute, durchsicht_fetchall, durchsicht_fetchone, init_db, volk_fetchall, volk_fetchone,
};

// to input a string
fn inp_str(prompt: &str) -> String {
    let mut line = String::new();
    println!("{}", prompt);

    std::io::stdin().read_line(&mut line).unwrap();
    let input = line.trim().to_string();
    println!("{}", &input);
    input
}

// to input a boolean
fn inp_bool(prompt: &str) -> bool {
    let mut line = String::new();
    println!("{}", prompt);
    std::io::stdin().read_line(&mut line).unwrap();
    let input = line.trim().to_string();
    let bool_ = match input.as_str() {
        "1" => true,
        "0" => false,
        _ => false,
    };
    println!("{}", &bool_);
    bool_
}

// to input a number i16
fn inp_number(prompt: &str) -> i16 {
    let mut line = String::new();
    println!("{}", prompt);
    std::io::stdin().read_line(&mut line).unwrap();
    let input = line.trim().parse::<i16>().unwrap();
    println!("{}", &input);
    input
}

// console app
fn main() {
    let mut ds: Durchsicht = Default::default();
    let input = inp_str("Datum?  JJJJ-MM-TT");
    ds.datum = input;
    let input = inp_str("Volk?  Volk 99");
    ds.volk = input;
    let input = inp_bool("Königin?  1 | 0");
    ds.koenigin = input;
    let input = inp_bool("Stifte?  1 | 0");
    ds.stifte = input;
    let input = inp_bool("offene Brut?  1 | 0");
    ds.offene = input;
    let input = inp_bool("verdeckelte Brut?  1 | 0");
    ds.verdeckelte = input;
    let input = inp_bool("Weiselzelle?  1 | 0");
    ds.weiselzelle = input;
    let input = inp_bool("Spielnäpfe?  1 | 0");
    ds.spielnaepfe = input;
    let input = inp_number("Sanftmut?  1 - 5");
    ds.sanftmut = input;
    let input = inp_number("Volkstärke?  1 - 5");
    ds.volksstaerke = input;
    let input = inp_number("Anzahl Brutwaben?  9");
    ds.anz_brutwaben = input;
    let input = inp_str("Bemerkungen?");
    ds.memo = input;
    let sql = Durchsicht::ds_to_sql(&ds);
    println!("{}", &sql);
    let db = init_db();
    let lines = db_execute(sql.as_str(), db);
    println!("Es wurde(n) {} Zeile(n) hinzugefügt.", lines);
}
