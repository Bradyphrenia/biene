mod feeding_mod;

use crate::feeding_mod::{
    brutto_weight, feed_need, netto_weight, Count, WarreCounts, WarreWeights, Weight,
};

pub fn main() {
    let testww = WarreWeights::new();
    let testwc = WarreCounts::new();
    let wweight = netto_weight(testww, testwc, false);
    let bweight = brutto_weight(testww, testwc, false);
    let feedw = feed_need(bweight, 25.0);
    println!("Nettogewicht der Warré-Beute ohne Fütterer: {wweight}");
    println!("Sollgewicht der Warré-Beute ohne Fütterer nach dem Einfüttern: {bweight}");
    println!("Futter notwendig bei einem aktuellen Gewicht von 25 kg: {feedw}")
}
