mod feeding;
use feeding::calculations::{
    brutto_weight, feed_need, feed_present, netto_weight, SetFeeder, SetZargeCount, WarreCounts,
    WarreWeights,
};
use crate::feeding::calculations::{DadantCounts, DadantWeights, NormalmassCounts, NormalmassWeights};

pub fn main() {
    let testww = WarreWeights::new();
    let mut testwc = WarreCounts::new();
    testwc = testwc.set_zarge_count(3).set_feeder(false);
    dbg!(&testwc);
    let wweight = netto_weight(testww, testwc);
    let fw = feed_present(testww, testwc, 35.0);
    let bweight = brutto_weight(testww, testwc);
    let feedw = feed_need(bweight, 35.0);
    println!("Nettogewicht der Warré-Beute ohne Fütterer: {wweight} kg");
    println!("Futter noch in der Warré-Beute ohne Fütterer: {fw} kg");
    println!("Sollgewicht der Warré-Beute ohne Fütterer nach dem Einfüttern: {bweight} kg");
    println!("Futter notwendig bei einem aktuellen Gewicht von 28 kg: {feedw} Liter 1:1");
    let testdd = DadantWeights::new();
    let mut testddc = DadantCounts::new();
    testddc = testddc.set_feeder(false);
    let bddsw = brutto_weight(testdd,testddc);
    println!("Sollgewicht der Dadant-Beute ohne Fütterer nach dem Einfüttern: {bddsw} kg");
    let testdn = NormalmassWeights::new();
    let mut testdnc = NormalmassCounts::new();
    testdnc = testdnc.set_feeder(false);
    let bdnsw = brutto_weight(testdn,testdnc);
    println!("Sollgewicht der Normalmaß-Beute ohne Fütterer nach dem Einfüttern: {bdnsw} kg");
}
