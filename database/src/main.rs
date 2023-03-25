use postgres::{Client, Error, NoTls, Row};

fn init_db() -> Client {
    // TODO: () <- parameters for database access
    let client = match Client::connect("postgresql://postgres:postgres@localhost:5432/biene", NoTls)
    {
        Ok(client) => client,
        Err(_e) => panic!("{}", _e), // database out of reach -> panic
    };
    return client;
}

struct Volk {
    id: i32,
    volk: String,
    nummer: i32,
    koenigin: String,
    erstellt: String,
    aufgeloest: String,
    typ: String,
    raehmchenmass: String,
    stand: String,
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

fn volk_fetchone(sql: &str, mut client: Client) -> Volk {
    let default_vk: Volk = Volk {
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
    let row = match client.query_one(sql, &[]) {
        Ok(row) => row,
        Err(_e) => return default_vk, // return a default strukt
    };
    let volk = Volk::from(row);
    return volk; // return a strukt
}

struct Durchsicht {
    id: i32,
    datum: String,
    volk: String,
    koenigin: bool,
    stifte: bool,
    offene: bool,
    verdeckelte: bool,
    weiselzelle: bool,
    spielnaepfe: bool,
    sanftmut: i16,
    volksstaerke: i16,
    anz_brutwaben: i16,
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

fn durchsicht_fetchone(sql: &str, mut client: Client) -> Durchsicht {
    let default_ds = Durchsicht {
        id: 0,
        datum: "2020-01-01".to_string(),
        volk: "".to_string(),
        koenigin: false,
        stifte: false,
        offene: false,
        verdeckelte: false,
        weiselzelle: false,
        spielnaepfe: false,
        sanftmut: 0,
        volksstaerke: 0,
        anz_brutwaben: 0,
    };
    let row = match client.query_one(sql, &[]) {
        Ok(row) => row,
        Err(_e) => return default_ds, // return a default strukt
    };
    let durchsicht = Durchsicht::from(row);
    return durchsicht; // return a strukt
}

fn main() {
    let test3 = volk_fetchone("SELECT id,volk,nummer,koenigin,erstellt::varchar,aufgeloest::varchar,typ,raehmchenmass, stand FROM volk", init_db());
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
    let test4 = durchsicht_fetchone("SELECT id,datum::varchar,volk,koenigin,stifte,offene,verdeckelte,weiselzelle,spielnaepfe,sanftmut,volksstaerke,anz_brutwaben FROM durchsicht", init_db());
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
