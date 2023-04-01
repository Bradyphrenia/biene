mod feeding_mod;

use crate::feeding_mod::{
    brutto_weight, feed_need, netto_weight, Count, WarreCounts, WarreWeights, Weight,
};

pub fn main() {
    let testww = feeding_mod::WarreWeights::new();
    let testwc = WarreCounts::new();
    let wweight = netto_weight(testww, testwc, false);
    let bweight = brutto_weight(testww, testwc, false);
    let feedw = feed_need(bweight, 25.0);
    println!("Nettogewicht der Warré-Beute ohne Fütterer: {}", wweight);
    println!(
        "Sollgewicht der Warré-Beute nach dem Einfüttern: {}",
        bweight
    );
    println!("Futter noch zu geben bei Gewicht von 25 kg: {}", feedw)
}
