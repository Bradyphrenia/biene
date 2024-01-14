use crate::cli::small_cli::{input_bool, input_number, input_string};
use crate::hivetypes::calculations::{
    BruttoWeight, FeedNeed, FeedPresent, HiveName, HiveTypes, NettoWeight, New, SetCurrentWeight,
    SetFeeder, Types,
};

mod cli;
mod hivetypes;

fn feed_str(present: bool) -> String {
    let feed = match present {
        true => "mit",
        false => "ohne",
    };
    return feed.to_string();
}

/// Ask user to define hive type
fn choose_hive_type() -> Types {
    loop {
        let type_ = input_string("(d)adant, (n)ormalmaß, (w)arré?");
        match type_.as_str() {
            "d" => return Types::dadant,
            "n" => return Types::deutschnormal,
            "w" => return Types::warre,
            _ => continue,
        };
    }
}

fn feeder_present() -> bool {
    loop {
        let feed_ = input_string("Fütterer (j,n)?");
        match feed_.as_str() {
            "j" => return true,
            "n" => return false,
            _ => continue,
        };
    }
}

/// Main function that calculates and prints various hive information for different types of hives.
pub fn main() {
    let hive_type = choose_hive_type();
    let feeder = feeder_present();
    let current_weight = input_number("kg?", 0, 100);
    let mut hive_ = HiveTypes::new(hive_type);
    let name = hive_.return_hive_name();
    hive_.set_feeder(feeder); // feeder?
    hive_.set_current_weight(current_weight as f32); // set current weight to ... kg
    let weight = hive_.netto_weight();
    let mit_ohne = feed_str(feeder);
    println!("Nettogewicht der {name}-Beute {mit_ohne} Fütterer: {weight} kg");
    let present = hive_.feed_present(); // calculate the amount of feed actually present
    println!("Futter noch in der {name}-Beute ohne Fütterer: {present} kg");
    let brutto = hive_.brutto_weight(); // calculate what a hive should weight if already ready for wintering
    println!("Sollgewicht der {name}-Beute ohne Fütterer nach dem Einfüttern: {brutto} kg");
    let need = hive_.feed_need(); // calculate the amount of sugar sirup 1:1
    println!(
        "Futter notwendig bei einem aktuellen Gewicht von {current_weight} kg: {need} Liter 1:1"
    );
    println!();
}
