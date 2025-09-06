use chrono::{Duration, Local, NaiveDate};
use std::env;

struct QueenRaring {
    larva: NaiveDate,
    caging: NaiveDate,
    transfer: NaiveDate,
    emerge: NaiveDate,
}

impl QueenRaring {
    fn from_start(dt: NaiveDate) -> Self {
        Self {
            larva: dt,
            caging: dt + Duration::days(5),
            transfer: dt + Duration::days(10),
            emerge: dt + Duration::days(11),
        }
    }
}

fn parse_input_date() -> Result<NaiveDate, String> {
    let mut args = env::args().skip(1).collect::<Vec<_>>();

    if args.is_empty() {
        // No args: use today's local date
        let today = Local::now().date_naive();
        return Ok(today);
    }

    if args.len() == 1 {
        let s = &args[0];
        // Try common formats
        if let Ok(d) = NaiveDate::parse_from_str(s, "%Y-%m-%d") {
            return Ok(d);
        }
        if let Ok(d) = NaiveDate::parse_from_str(s, "%Y/%m/%d") {
            return Ok(d);
        }
        if let Ok(d) = NaiveDate::parse_from_str(s, "%Y.%m.%d") {
            return Ok(d);
        }
        return Err(format!(
            "Invalid date format '{}'. Use YYYY-MM-DD (e.g., 2025-07-14) or provide three args: YEAR MONTH DAY.",
            s
        ));
    }

    if args.len() == 3 {
        let year: i32 = args[0].parse().map_err(|_| "YEAR must be an integer".to_string())?;
        let month: u32 = args[1].parse().map_err(|_| "MONTH must be an integer".to_string())?;
        let day: u32 = args[2].parse().map_err(|_| "DAY must be an integer".to_string())?;
        if let Some(d) = NaiveDate::from_ymd_opt(year, month, day) {
            return Ok(d);
        } else {
            return Err("Provided YEAR MONTH DAY do not form a valid date".to_string());
        }
    }

    Err("Usage: queen [YYYY-MM-DD] or queen [YEAR MONTH DAY] (no args uses today's date)".to_string())
}

fn main() {
    match parse_input_date() {
        Ok(start) => {
            let breeding = QueenRaring::from_start(start);
            println!("Starting series on {}.", breeding.larva);
            println!("Caging cells on {}.", breeding.caging);
            println!("Transferring cells allowed on {}.", breeding.transfer);
            println!("Queens emerging on {}.", breeding.emerge);
        }
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }
}
