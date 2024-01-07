use crate::hivetypes::calculations::{
    BruttoWeight, FeedNeed, FeedPresent, HiveName, HiveTypes, NettoWeight, New, SetCurrentWeight,
    SetFeeder, Types,
};

mod hivetypes;

/// Main function that calculates and prints various hive information for different types of hives.
pub fn main() {
    let weight = 30.0;
    for type_ in [Types::warre, Types::dadant, Types::deutschnormal] {
        let mut test_hive = HiveTypes::new(type_);
        let name = test_hive.return_hive_name();
        test_hive.set_feeder(false); // no feeder
        test_hive.set_current_weight(weight); // set current weight to ... kg
        let weight = test_hive.netto_weight();
        println!("Nettogewicht der {name}-Beute ohne Fütterer: {weight} kg");
        let present = test_hive.feed_present(); // calculate the amount of feed actually present
        println!("Futter noch in der {name}-Beute ohne Fütterer: {present} kg");
        let brutto = test_hive.brutto_weight(); // calculate what a hive should weight if already ready for wintering
        println!("Sollgewicht der {name}-Beute ohne Fütterer nach dem Einfüttern: {brutto} kg");
        let need = test_hive.feed_need(); // calculate the amount of sugar sirup 1:1
        println!("Futter notwendig bei einem aktuellen Gewicht von {weight} kg: {need} Liter 1:1");
        println!();
    }
}
