use postgres::{Client, Error, NoTls, Row};

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

fn database_fetchone(sql: &str) -> Result<Volk, Error> {
    let mut client = Client::connect("postgresql://postgres:postgres@localhost:5432/biene", NoTls)?;
    let row = client.query_one(sql, &[])?;
    let volk = Volk::from(row);
    Ok(volk) // return a strukt
}

fn main() {
    let test3 = database_fetchone("SELECT id,volk,nummer,koenigin,erstellt::varchar,aufgeloest::varchar,typ,raehmchenmass, stand FROM volk").unwrap();
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
