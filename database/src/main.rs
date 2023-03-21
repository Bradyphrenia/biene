use postgres::{Client, Error, NoTls};


fn database_query(sql: &str) -> Result<i32, Error> {
    let mut client = Client::connect("postgresql://postgres:postgres@localhost:5432/biene", NoTls)?;

    for row in client.query(sql, &[])? {
        let id: &str = row.get(0);
        let name: &str = row.get(1);
        let yard: &str = row.get(2);
        println!("id: {} name: {} yard: {}", id, name, yard);
    };

    Ok(12)
}

fn main() {
    let test = database_query("select * from hives;").unwrap();
    println!("{}", test)
}