use chrono::{Duration, NaiveDate};

fn main() {
    let dt = NaiveDate::from_ymd_opt(2023, 7, 3).unwrap();
    println!("Caging cells on {}.", dt + Duration::days(5));
    println!("Transferring cells on {}.", dt + Duration::days(10));
    println!("Queens emerging on {}.", dt + Duration::days(11));
}
