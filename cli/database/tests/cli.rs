use std::io::{BufRead, Write};

use rstest::rstest;

use database::cli::{date_string, input_number};
use database::dbase::{db_execute, init_db};

#[test]
fn works() {
    let db = init_db();
    let sql: String = "SELECT * FROM durchsicht WHERE id = 155;".to_string(); // one dataset!!!
    let lines = db_execute(sql.as_str(), db);
    assert_eq!(lines, 1);
}

#[rstest]
#[case("2023-12-29", true)]
#[case("23-12-29", false)]
fn test_check_date(#[case] input: &str, #[case] expected: bool) {
    assert_eq!(date_string(input), expected);
}

#[rstest]
#[case(("10", 1, 10), 10)]
#[should_panic]
#[case(("10", 1, 3), 10)]
fn test_check_number(#[case] input: (&str, i16, i16), #[case] expected: i16) {
    assert_eq!(input_number(input.0, input.1, input.2), expected);
}
