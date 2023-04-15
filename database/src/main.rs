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

fn inp_number(prompt: &str) -> i8 {
    let mut line = String::new();
    println!("{}", prompt);
    std::io::stdin().read_line(&mut line).unwrap();
    let input = line.trim().to_string();
    input.parse::<i8>().unwrap()
}

fn main() {
    let test1 = inp_str("Volk? ");
    dbg!(test1);
    let test2 = inp_bool("Königin? ");
    dbg!(test2);
    let test3 = inp_number("Anzahl Brutwaben? ");
    dbg!(test3);
}
