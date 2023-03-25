use postgres::{Client, NoTls, Row};

pub fn init_db() -> Client {
    // TODO?: () <- parameters for database access
    let client = match Client::connect("postgresql://postgres:postgres@localhost:5432/biene", NoTls)
    {
        Ok(client) => client,
        Err(_e) => panic!("{}", _e), // database out of reach -> ok to panic
    };
    return client;
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

pub fn volk_fetchone(sql: &str, mut client: Client) -> Volk {
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

pub fn durchsicht_fetchone(sql: &str, mut client: Client) -> Durchsicht {
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
