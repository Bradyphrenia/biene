use chrono::NaiveDate;

// small cli :-)
// to input a string
pub fn input_string(prompt: &str) -> String {
    let mut line = String::new();
    match std::io::stdin().read_line(&mut line) {
        Ok(_) => return line.trim().to_string(),
        Err(e) => panic!("Error reading input: {}", e),
    }
}

//to input a valid date string
pub fn input_date(prompt: &str) -> String {
    let mut input = input_string(prompt);
    loop {
        check_date(input)
    }
}

pub fn check_date(input: String) -> String {
    // throw error on invalid input
    match NaiveDate::parse_from_str(&input, "%Y-%m-%d") {
        Ok(_) => return input,
        Err(_) => return String::new(),
    }
}

// to input a number i16
pub fn input_number(prompt: &str, min: i16, max: i16) -> i16 {
    let mut input = input_string(prompt);
    loop {
        check_number(input, min, max)
    }
}

pub fn check_number(input: String, min: i16, max: i16) -> i16 {
    match input.parse::<i16>() {
        Ok(ok) if ok >= min && ok <= max => ok,
        _ => panic!("Invalid number: {}", input),
    }
}

// to input a boolean
pub fn input_bool(prompt: &str) -> bool {
    let input = input_number(prompt, 0, 1);
    return if let 1 = input {
        let bool_ = true;
        bool_
    } else {
        let bool_ = false;
        bool_
    };
}

// for testing a date part for being in the right range
fn date_part(part: &str, min: i16, max: i16) -> bool {
    let test = &part.parse::<i16>();
    match test {
        Ok(ok) => {
            if *ok < min || *ok > max {
                return false;
            }
        }
        Err(_e) => return false,
    }
    true
}

// tests the date string for being valid
fn date_string(ds: &str) -> bool {
    if ds.len() < 10 {
        return false;
    };
    let year = &ds[..4];
    let month = &ds[5..7];
    let day = &ds[8..10];
    let del1 = &ds[4..5];
    let del2 = &ds[7..8];
    if del1 != "-" {
        return false;
    }
    if del2 != "-" {
        return false;
    }
    if date_part(year, 2020, 2030) == false {
        return false;
    }
    if date_part(month, 1, 12) == false {
        return false;
    }
    if date_part(day, 1, 31) == false {
        return false;
    }
    true
}
