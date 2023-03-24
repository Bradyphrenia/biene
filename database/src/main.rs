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

fn volk_fetchone(sql: &str, mut client: Client) -> Result<Volk, Error> {
    let row = client.query_one(sql, &[])?;
    let volk = Volk::from(row);
    Ok(volk) // return a strukt
}

pub struct Durchsicht {
    pub id: i32,
    pub datum: String,
    pub volk: String,
    pub koenigin: bool,
    pub stifte: bool,
    pub offene: bool,
    pub verdeckelte: bool,
    pub weiselzelle: bool,
    pub spielnaepfe: bool,
    pub sanftmut: i16,
    pub volksstaerke: i16,
    pub anz_brutwaben: i16,
}

impl From<Row> for Durchsicht {
    fn from(row: Row) -> Self {
        Self {
            id: row.get("id"),
            datum: row.get("datum"),
            volk: row.get("volk"),
            koenigin: row.get("koenigin"),
            stifte: row.get("stifte"),
            offene: row.get("offene"),
            verdeckelte: row.get("verdeckelte"),
            weiselzelle: row.get("weiselzelle"),
            spielnaepfe: row.get("spielnaepfe"),
            sanftmut: row.get("sanftmut"),
            volksstaerke: row.get("volksstaerke"),
            anz_brutwaben: row.get("anz_brutwaben"),
        }
    }
}

fn init_db() -> Result<Client, Error> {
    let mut client =
        match Client::connect("postgresql://postgres:postgres@localhost:5432/biene", NoTls) {
            Ok(client_) => client_,
            Err(e) => todo!(),
        };
    Ok(client)
}

fn durchsicht_fetchone(sql: &str, mut client: Client) -> Result<Durchsicht, Error> {
    let row = client.query_one(sql, &[])?;
    let durchsicht = Durchsicht::from(row);
    Ok(durchsicht) // return a strukt
}

fn main() {
    let test3 = volk_fetchone("SELECT id,volk,nummer,koenigin,erstellt::varchar,aufgeloest::varchar,typ,raehmchenmass, stand FROM volk", init_db().unwrap()).unwrap();
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
    let test4 = durchsicht_fetchone("SELECT id,datum::varchar,volk,koenigin,stifte,offene,verdeckelte,weiselzelle,spielnaepfe,sanftmut,volksstaerke,anz_brutwaben FROM durchsicht", init_db().unwrap()).unwrap();
    println!(
        "{} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} ",
        test4.id,
        test4.datum,
        test4.volk,
        test4.koenigin,
        test4.stifte,
        test4.offene,
        test4.verdeckelte,
        test4.weiselzelle,
        test4.spielnaepfe,
        test4.sanftmut,
        test4.volksstaerke,
        test4.anz_brutwaben,
    )
}
