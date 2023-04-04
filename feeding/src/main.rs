mod feeding;
use feeding::calculations::{WarreWeights, WarreCounts, SetZargeCount, SetFeeder, netto_weight, feed_need, feed_present, brutto_weight};

pub fn main() {
    let testww = WarreWeights::new();
    let mut testwc = WarreCounts::new();
    testwc = testwc.set_zarge_count(3).set_feeder(false);
    dbg!(&testwc);
    let wweight = netto_weight(testww, testwc);
    let fw = feed_present(testww, testwc, 28.0);
    let bweight = brutto_weight(testww, testwc);
    let feedw = feed_need(bweight, 28.0);
    println!("Nettogewicht der Warré-Beute ohne Fütterer: {wweight} kg");
    println!("Futter noch in der Warré-Beute ohne Fütterer: {fw} kg");
    println!("Sollgewicht der Warré-Beute ohne Fütterer nach dem Einfüttern: {bweight} kg");
    println!("Futter notwendig bei einem aktuellen Gewicht von 28 kg: {feedw} Liter 1:1")
}
