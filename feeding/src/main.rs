use crate::feeding_mod::{init_dadant_count, init_dadant_weight, netto_weight, Count, Weight};

mod feeding_mod;

fn main() {
    let testw: Weight = init_dadant_weight();
    let testc: Count = init_dadant_count();
    let w = netto_weight(testw, testc, false);
    println!("{}", w)
}
