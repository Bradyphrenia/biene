use chrono::{Duration, NaiveDate};

fn main() {
    let breeding = queen_raring(2023, 7, 2);
    println!("Caging cells on {}.", breeding.0);
    println!("Transferring cells allowed on {}.", breeding.1);
    println!("Queens emerging on {}.", breeding.2);
}

fn queen_raring(year: i32, month: u32, day: u32) -> (NaiveDate, NaiveDate, NaiveDate) {
    let dt = NaiveDate::from_ymd_opt(year, month, day).unwrap();
    (
        dt + Duration::days(5),
        dt + Duration::days(10),
        dt + Duration::days(11),
    )
}
