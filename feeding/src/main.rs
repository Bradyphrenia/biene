mod feeding_mod;

use crate::feeding_mod::{brutto_weight, feed_need, netto_weight, Count, SetCount, WarreCounts, WarreWeights, Weight, SetFeeder, SetZargeCount};

pub fn main() {
    let testww = WarreWeights::new();
    let mut testwc = WarreCounts::new();
    testwc = testwc.set_zarge_count(2);
    testwc = testwc.set_feeder(false);
    dbg!(&testwc);
    let wweight = netto_weight(testww, testwc);
    let bweight = brutto_weight(testww, testwc);
    let feedw = feed_need(bweight, 28.0);
    println!("Nettogewicht der Warré-Beute ohne Fütterer: {wweight} kg");
    println!("Sollgewicht der Warré-Beute ohne Fütterer nach dem Einfüttern: {bweight} kg");
    println!("Futter notwendig bei einem aktuellen Gewicht von 28 kg: {feedw} Liter")
}

