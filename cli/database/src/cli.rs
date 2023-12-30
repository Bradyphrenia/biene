// small cli :-)
// to input a string
pub fn input_string(prompt: &str) -> String {
    let mut line = String::new();
    println!("{}", prompt);
    std::io::stdin()
        .read_line(&mut line)
        .expect("Error reading input!");
    let input = line.trim().to_string();
    input
}

//to input a valid date string
pub fn input_date(prompt: &str) -> String {
    let mut input = String::new();
    loop {
        input = input_string(prompt);
        if date_string(&input) {
            return input.to_string();
        }
        input = "".to_string();
    }
}

// to input a number i16
pub fn input_number(prompt: &str, min: i16, max: i16) -> i16 {
    let mut input = String::new();
    let mut ret_value: i16;
    loop {
        println!("{}", prompt);
        std::io::stdin()
            .read_line(&mut input)
            .expect("Error reading input!");
        let test = &input.trim().parse::<i16>();
        match &test {
            Ok(ok) => ret_value = *ok,
            Err(_e) => ret_value = -1,
        }
        if (ret_value != -1) && (ret_value >= min) && (ret_value <= max) {
            break;
        }
        input = "".to_string();
    }
    return ret_value;
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
pub fn date_string(ds: &str) -> bool {
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

