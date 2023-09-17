mod feeding;
use crate::feeding::calculations::{
    DadantCounts, DadantWeights, NormalmassCounts, NormalmassWeights,
};
use feeding::calculations::{
    brutto_weight, feed_need, feed_present, netto_weight, SetFeeder, SetZargeCount, WarreCounts,
    WarreWeights,
};

pub fn main() {
    let testwrw = WarreWeights::new();
    let mut testwrc = WarreCounts::new();
    testwrc = testwrc.set_zarge_count(3).set_feeder(false);
    let wrweight = netto_weight(testwrw, testwrc);
    let fwr = feed_present(testwrw, testwrc, 35.0);
    let bwrweight = brutto_weight(testwrw, testwrc);
    let feedwr = feed_need(bwrweight, 35.0);
    println!("Nettogewicht der Warré-Beute ohne Fütterer: {wrweight} kg");
    println!("Futter noch in der Warré-Beute ohne Fütterer: {fwr} kg");
    println!("Sollgewicht der Warré-Beute ohne Fütterer nach dem Einfüttern: {bwrweight} kg");
    println!("Futter notwendig bei einem aktuellen Gewicht von 35 kg: {feedwr} Liter 1:1");
    println!();
    let testddw = DadantWeights::new();
    let mut testddc = DadantCounts::new();
    testddc = testddc.set_feeder(false);
    let ddweight = netto_weight(testddw, testddc);
    let fdd = feed_present(testddw, testddc, 35.0);
    let bddweight = brutto_weight(testddw, testddc);
    let feeddd = feed_need(bddweight, 35.0);
    println!("Nettogewicht der Dadant-Beute ohne Fütterer: {ddweight} kg");
    println!("Futter noch in der Dadant-Beute ohne Fütterer: {fdd} kg");
    println!("Sollgewicht der Dadant-Beute ohne Fütterer nach dem Einfüttern: {bddweight} kg");
    println!("Futter notwendig bei einem aktuellen Gewicht von 35 kg: {feeddd} Liter 1:1");
    println!();
    let testdnw = NormalmassWeights::new();
    let mut testdnc = NormalmassCounts::new();
    testdnc = testdnc.set_feeder(false);
    let dnweight = netto_weight(testdnw, testdnc);
    let fdn = feed_present(testdnw, testdnc, 35.0);
    let bdnw = brutto_weight(testdnw, testdnc);
    let feeddn = feed_need(bdnw, 35.0);
    println!("Nettogewicht der Normalmaß-Beute ohne Fütterer: {dnweight} kg");
    println!("Futter noch in der Normalmaß-Beute ohne Fütterer: {fdn} kg");
    println!("Sollgewicht der Normalmaß-Beute ohne Fütterer nach dem Einfüttern: {bdnw} kg");
    println!("Futter notwendig bei einem aktuellen Gewicht von 35 kg: {feeddn} Liter 1:1");
}
