use chrono::{Duration, NaiveDate};

struct QueenRaring {
    larva: NaiveDate,
    caging: NaiveDate,
    transfer: NaiveDate,
    emerge: NaiveDate,
}

fn main() {
    let mut breeding = QueenRaring {
        larva: NaiveDate::parse_from_str("2020-1-1", "%Y-%m-%d").unwrap(),
        caging: NaiveDate::parse_from_str("2020-1-1", "%Y-%m-%d").unwrap(),
        transfer: NaiveDate::parse_from_str("2020-1-1", "%Y-%m-%d").unwrap(),
        emerge: NaiveDate::parse_from_str("2020-1-1", "%Y-%m-%d").unwrap(),
    };
    breeding.queen_raring(2023, 7, 2);
    println!("Started series on {}.", breeding.larva);
    println!("Caging cells on {}.", breeding.caging);
    println!("Transferring cells allowed on {}.", breeding.transfer);
    println!("Queens emerging on {}.", breeding.emerge);
}

impl QueenRaring {
    fn queen_raring(&mut self, year: i32, month: u32, day: u32) -> () {
        match NaiveDate::from_ymd_opt(year, month, day) {
            Some(dt) => {
                self.larva = dt;
                self.caging = dt + Duration::days(5); //Caging cells on
                self.transfer = dt + Duration::days(10); //Transferring cells allowed on
                self.emerge = dt + Duration::days(11); //Queens emerging on
            }
            None => (todo!()),
        };
    }
}
