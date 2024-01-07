use crate::hivetypes::calculations::{
    BruttoWeight, FeedNeed, FeedPresent, HiveTypes, NettoWeight, New, SetCurrentWeight, SetFeeder,
    Types,
};

mod hivetypes;

fn type_name(types: Types) -> String {
    match types {
        Types::warre => return "Warré".to_string(),
        Types::deutschnormal => return "Deutsch Normalmass".to_string(),
        Types::dadant => return "Dadant".to_string(),
    }
}
pub fn main() {
    for type_ in [Types::warre, Types::dadant, Types::deutschnormal] {
        let name = type_name(type_);
        let mut test_hive = HiveTypes::new(type_);
        test_hive.set_feeder(false); // no feeder
        test_hive.set_current_weight(28.0); // set current weight to 28 kg
        let weight = test_hive.netto_weight();
        println!("Nettogewicht der {name}-Beute ohne Fütterer: {weight} kg");
        let present = test_hive.feed_present(); // calculate the amount of feed actual present
        println!("Futter noch in der {name}-Beute ohne Fütterer: {present} kg");
        let brutto = test_hive.brutto_weight(); // calculate what a hive should weight if already full
        println!("Sollgewicht der {name}-Beute ohne Fütterer nach dem Einfüttern: {brutto} kg");
        let need = test_hive.feed_need(); // calculate the amount of sugar sirup 1:1
        println!("Futter notwendig bei einem aktuellen Gewicht von 28 kg: {need} Liter 1:1");
        println!();
    }
}
