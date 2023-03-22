use postgres::{Client, Error, types, NoTls};

fn database_query(sql: &str) -> Result<i32, Error> {
    let mut client = Client::connect("postgresql://postgres:postgres@localhost:5432/biene", NoTls)?;
    for row in client.query(sql, &[])? {
        let id: &str = row.get(0);
        let name: &str = row.get(1);
        let yard: &str = row.get(2);
        println!("id: {} name: {} yard: {}", id, name, yard);
    };
    Ok(12)  // return a test value
}

fn database_execute(sql: &str) -> Result<&str, Error> {
    let mut client = Client::connect("postgresql://postgres:postgres@localhost:5432/biene", NoTls)?;
    for row in client.query(sql, &[])? {
        for (col_idx, _col) in row.columns().iter().enumerate() {
            let _val: &str =
                match row.get(col_idx) {  //now it works with pattern matching :-)
                    Some(val) => val,
                    None => ""
                };
            print!("{} ", _val);
        }
    };
    Ok("13")   // return a test value
}


fn main() {
    let test = database_query("select * from hives;").unwrap();
    println!("{}", test);
    let test2 = database_execute("select * from hives;").unwrap();
    println!("{}", test2)
}

