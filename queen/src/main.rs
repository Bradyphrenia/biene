use chrono::{Duration, NaiveDate};

fn main() {
    let breeding = queen_raring(2023, 13, 2);
    println!("Caging cells on {}.", breeding.0);
    println!("Transferring cells allowed on {}.", breeding.1);
    println!("Queens emerging on {}.", breeding.2);
}

fn queen_raring(year: i32, month: u32, day: u32) -> (NaiveDate, NaiveDate, NaiveDate) {
    let dt = match NaiveDate::from_ymd_opt(year, month, day) {
        Some(dt) =>
            (
                dt + Duration::days(5), //Caging cells on
                dt + Duration::days(10), //Transferring cells allowed on
                dt + Duration::days(11), //Queens emerging on
            ),
        None => (NaiveDate::parse_from_str("2020-1-1", "%Y-%m-%d").unwrap(), NaiveDate::parse_from_str("2020-1-1", "%Y-%m-%d").unwrap(), NaiveDate::parse_from_str("2020-1-1", "%Y-%m-%d").unwrap())
    };
    dt
}
