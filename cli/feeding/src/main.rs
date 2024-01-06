use crate::hivetypes::calculations::{
    BruttoWeight, FeedPresent, HiveTypes, NettoWeight, New, SetCurrentWeight, SetFeeder, Types,
};

pub mod feeding;
mod hivetypes;

pub fn main() {
    let mut testwrw = HiveTypes::new(Types::warre);
    testwrw.set_feeder(false);
    testwrw.set_current_weight(40.0);
    let weight = testwrw.netto_weight();
    println!("Nettogewicht der Warré-Beute ohne Fütterer: {weight} kg");
    // println!("Futter noch in der Warré-Beute ohne Fütterer: {fwr} kg");
    // println!("Sollgewicht der Warré-Beute ohne Fütterer nach dem Einfüttern: {bwrweight} kg");
    // println!("Futter notwendig bei einem aktuellen Gewicht von 35 kg: {feedwr} Liter 1:1");
    // println!();
    // let testddw = DadantWeights::new();
    // let mut testddc = DadantCounts::new();
    // testddc = testddc.set_feeder(false);
    // let ddweight = netto_weight(testddw, testddc);
    // let fdd = feed_present(testddw, testddc, 36.0);
    // let bddweight = brutto_weight(testddw, testddc);
    // let feeddd = feed_need(bddweight, 36.0);
    // println!("Nettogewicht der Dadant-Beute ohne Fütterer: {ddweight} kg");
    // println!("Futter noch in der Dadant-Beute ohne Fütterer: {fdd} kg");
    // println!("Sollgewicht der Dadant-Beute ohne Fütterer nach dem Einfüttern: {bddweight} kg");
    // println!("Futter notwendig bei einem aktuellen Gewicht von 36 kg: {feeddd} Liter 1:1");
    // println!();
    // let testdnw = NormalmassWeights::new();
    // let mut testdnc = NormalmassCounts::new();
    // testdnc = testdnc.set_feeder(false);
    // let dnweight = netto_weight(testdnw, testdnc);
    // let fdn = feed_present(testdnw, testdnc, 36.0);
    // let bdnw = brutto_weight(testdnw, testdnc);
    // let feeddn = feed_need(bdnw, 36.0);
    // println!("Nettogewicht der Normalmaß-Beute ohne Fütterer: {dnweight} kg");
    // println!("Futter noch in der Normalmaß-Beute ohne Fütterer: {fdn} kg");
    // println!("Sollgewicht der Normalmaß-Beute ohne Fütterer nach dem Einfüttern: {bdnw} kg");
    // println!("Futter notwendig bei einem aktuellen Gewicht von 36 kg: {feeddn} Liter 1:1");
}
