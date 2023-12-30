use std::io::{self, BufRead, Write};

use rstest::rstest;

use database::cli::{input_date, check_date, check_number};
use database::dbase::{Durchsicht, db_execute, durchsicht_fetchall, durchsicht_fetchone, init_db, volk_fetchall, volk_fetchone,
};

#[test]
fn works() {
    let db = init_db();
    let sql: String = "SELECT * FROM durchsicht WHERE id = 155;".to_string(); // one dataset!!!
    let lines = db_execute(sql.as_str(), db);
    assert_eq!(lines, 1);
}

#[rstest]
#[case("2023-12-29".to_string(), "2023-12-29".to_string())]
#[case("23-12-29".to_string(), "23-12-29".to_string())]
#[case("-10-1-191".to_string(), String::new())]
fn test_check_date(#[case] input: String, #[case] expected: String) {
    assert_eq!(check_date(input), expected);
}

#[rstest]
#[case(("10".to_string(), 1, 15), 10)]
#[should_panic]
#[case(("10".to_string(), 1, 5), 10)]
fn test_check_number(#[case] input: (String, i16, i16), #[case] expected: i16) {
    assert_eq!(check_number(input.0, input.1, input.2), expected);
}
