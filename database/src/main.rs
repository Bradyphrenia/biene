use postgres::{Client, Error, NoTls, Row};

fn database_query(sql: &str) -> Result<i32, Error> {
    let mut client = Client::connect("postgresql://postgres:postgres@localhost:5432/biene", NoTls)?;
    for row in client.query(sql, &[])? {
        let id: &str = row.get(0);
        let name: &str = row.get(1);
        let yard: &str = row.get(2);
        println!("id: {} name: {} yard: {}", id, name, yard);
    }
    Ok(12) // return a test value
}

fn database_execute(sql: &str) -> Result<&str, Error> {
    let mut client = Client::connect("postgresql://postgres:postgres@localhost:5432/biene", NoTls)?;
    for row in client.query(sql, &[])? {
        for (col_idx, _col) in row.columns().iter().enumerate() {
            let _val: &str = match row.get(col_idx) {
                //now it works with pattern matching :-)
                Some(val) => val,
                None => "",
            };
            print!("{} ", _val);
        }
    }
    Ok("13") // return a test value
}

pub struct Volk {
    pub id: i32,
    pub volk: String,
    pub nummer: i32,
    pub koenigin: String,
    pub erstellt: String,
    pub aufgeloest: String,
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
            koenigin: match row.get("koenigin") {
                Some(val) => val,
                None => "".to_string(),
            },
            erstellt: match row.get("erstellt") {
                Some(val) => val,
                None => "".to_string(),
            },
            aufgeloest: match row.get("aufgeloest") {
                Some(val) => val,
                None => "".to_string(),
            },
            typ: row.get("typ"),
            raehmchenmass: row.get("raehmchenmass"),
            stand: row.get("stand"),
        }
    }
}

fn database_test(sql: &str) -> Result<Volk, Error> {
    let mut client = Client::connect("postgresql://postgres:postgres@localhost:5432/biene", NoTls)?;
    let mut result: Volk = Volk {
        id: 0,
        volk: "".to_string(),
        nummer: 0,
        koenigin: "".to_string(),
        erstellt: "".to_string(),
        aufgeloest: "".to_string(),
        typ: "".to_string(),
        raehmchenmass: "".to_string(),
        stand: "".to_string(),
    };
    for row in client.query(sql, &[])? {
        let volk = Volk::from(row);
        result = volk
    }
    Ok(result) // return a strukt
}

fn main() {
    let test = database_query("select * from hives;").unwrap();
    println!("{}", test);
    println!();
    let test2 = database_execute("select * from hives;").unwrap();
    println!("{}", test2);
    println!();
    let test3 = database_test("SELECT id,volk,nummer,koenigin,erstellt::varchar,aufgeloest::varchar,typ,raehmchenmass, stand FROM volk").unwrap();
    println!(
        "{} | {} | {} | {} | {} | {} | {} | {} | {} ",
        test3.id,
        test3.volk,
        test3.nummer,
        test3.koenigin,
        test3.erstellt,
        test3.aufgeloest,
        test3.typ,
        test3.raehmchenmass,
        test3.stand
    );
}
