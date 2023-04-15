use crate::database::dbase::Durchsicht;
use database::dbase::{
    durchsicht_fetchall, durchsicht_fetchone, init_db, volk_fetchall, volk_fetchone,
};

mod database;

fn inp_str(prompt: &str) -> String {
    let mut line = String::new();
    println!("{}", prompt);
    std::io::stdin().read_line(&mut line).unwrap();
    let input = line.trim().to_string();
    input
}

fn inp_bool(prompt: &str) -> bool {
    let mut line = String::new();
    println!("{}", prompt);
    std::io::stdin().read_line(&mut line).unwrap();
    let input = line.trim().to_string();
    match input.as_str() {
        "1" => true,
        _ => false,
    }
}

fn inp_number(prompt: &str) -> i16 {
    let mut line = String::new();
    println!("{}", prompt);
    std::io::stdin().read_line(&mut line).unwrap();
    let input = line.trim().to_string();
    input.parse::<i16>().unwrap()
}

fn main() {
    let mut ds: Durchsicht = Default::default();
    let input = inp_str("Datum?");
    ds.datum = input;
    let input = inp_str("Volk?");
    ds.volk = input;
    let input = inp_bool("Königin?");
    ds.koenigin = input;
    let input = inp_bool("Stifte?");
    ds.stifte = input;
    let input = inp_bool("offene Brut?");
    ds.offene = input;
    let input = inp_bool("verdeckelte Brut?");
    ds.verdeckelte = input;
    let input = inp_bool("Weiselzelle?");
    ds.weiselzelle = input;
    let input = inp_bool("Spielnäpfe?");
    ds.spielnaepfe = input;
    let input = inp_number("Sanftmut?");
    ds.sanftmut = input;
    let input = inp_number("Volkstärke?");
    ds.volksstaerke = input;
    let input = inp_number("Anzahl Brutwaben?");
    ds.anz_brutwaben = input;
    let input = inp_str("Bemerkungen?");
    ds.memo = input;
    dbg!(ds);
}
