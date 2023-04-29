mod database;

use crate::database::dbase::Durchsicht;
use database::dbase::{
    db_execute, durchsicht_fetchall, durchsicht_fetchone, init_db, volk_fetchall, volk_fetchone,
};

// small cli :-)
// to input a string
fn input_string(prompt: &str) -> String {
    let mut line = String::new();
    println!("{}", prompt);
    std::io::stdin().read_line(&mut line).unwrap();
    let input = line.trim().to_string();
    input
}

//to input a valid date string
fn input_date(prompt: &str) -> String {
    let mut input = String::new();
    loop {
        input = input_string(prompt);
        if date_string(&input) {
            return input.to_string();
        }
        input = "".to_string();
    }
}

// to input a number i16
fn input_number(prompt: &str, min: i16, max: i16) -> i16 {
    let mut input = String::new();
    let mut ret_value: i16;
    println!("{}", prompt);
    loop {
        std::io::stdin()
            .read_line(&mut input)
            .expect("Error reading input!");
        let test = &input.trim().parse::<i16>();
        match &test {
            Ok(ok) => ret_value = *ok,
            Err(_e) => ret_value = -1,
        }
        if (ret_value != -1) && (ret_value >= min) && (ret_value <= max) {
            break;
        }
        input = "".to_string();
    }
    return ret_value;
}

// to input a boolean
fn input_bool(prompt: &str) -> bool {
    let input = input_number(prompt, 0, 1);
    return if let 1 = input {
        let bool_ = true;
        bool_
    } else {
        let bool_ = false;
        bool_
    };
}

// tests the date string
fn date_string(ds: &str) -> bool {
    if ds.len() < 10 {
        return false;
    };
    let year = &ds[..4];
    let month = &ds[5..7];
    let day = &ds[8..10];
    let del1 = &ds[4..5];
    let del2 = &ds[7..8];
    if del1 != "-" {
        return false;
    }
    if del2 != "-" {
        return false;
    }
    let ret_y: i16;
    let ret_m: i16;
    let ret_d: i16;
    let ty = &year.parse::<i16>();
    match ty {
        Ok(ok) => ret_y = *ok,
        Err(_e) => return false,
    }
    if ret_y < 2020 || ret_y > 2030 {
        return false;
    }
    let tm = &month.parse::<i16>();
    match tm {
        Ok(ok) => ret_m = *ok,
        Err(_e) => return false,
    }
    if ret_m < 1 || ret_m > 12 {
        return false;
    }
    let td = &day.parse::<i16>();
    match td {
        Ok(ok) => ret_d = *ok,
        Err(_e) => return false,
    }
    if ret_d < 1 || ret_d > 31 {
        return false;
    }
    true
}

// console app
fn main() {
    let test = date_string("2020-01-01");
    println!("{}", test);
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
}
