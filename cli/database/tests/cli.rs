use database::dbase::Durchsicht;
use database::dbase::{
    db_execute, durchsicht_fetchall, durchsicht_fetchone, init_db, volk_fetchall, volk_fetchone,
};

#[test]
fn works() {
    let db = init_db();
    let sql: String = "SELECT * FROM durchsicht WHERE id = 155;".to_string(); // one dataset!!!
    let lines = db_execute(sql.as_str(), db);
    assert_eq!(lines, 1);
}
