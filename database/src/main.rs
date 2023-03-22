use postgres::{Client, Error, types, NoTls, Row};


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


pub struct Volk {
    pub id: i32,
    pub volk: String,
    pub nummer: i32,
    pub koenigin: Option<String>,
    //   pub erstellt: Date<T>,
    //   pub aufgeloest: Option<Date<T>>,
    pub typ: String,
    pub raehmchenmass: String,
    pub stand: String,
}

impl From<Row> for Volk {
    fn from(row: Row) -> Self {
        Self {
            id: row.get("id"),
            volk: row.get("volk"),
            nummer: row.get("nummer"),
            koenigin: row.get("koenigin"),
            //           erstellt: row.get("erstellt"),
            //          aufgeloest: row.get("aufgeloest"),
            typ: row.get("typ"),
            raehmchenmass: row.get("raehmchenmass"),
            stand: row.get("stand"),
        }
    }
}


fn database_test(sql: &str) -> Result<i32, Error> {
    let mut client = Client::connect("postgresql://postgres:postgres@localhost:5432/biene", NoTls)?;
    for row in client.query(sql, &[])? {
        let mut volk = Volk::from(row);
        println!("{} {} {} {} {} ", volk.id, volk.volk, volk.nummer, volk.typ, volk.raehmchenmass)
    }
    Ok(15)  // return a test value
}

fn main() {
    let test = database_query("select * from hives;").unwrap();
    println!("{}", test);
    let test2 = database_execute("select * from hives;").unwrap();
    println!("{}", test2);
    let test3 = database_test("SELECT * FROM volk").unwrap();
    println!("{}", test3);
}

