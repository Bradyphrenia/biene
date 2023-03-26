use postgres::{Client, NoTls, Row};

// initialize the database

pub fn init_db() -> Client {
    // TODO?: () <- parameters for database access
    let client = match Client::connect("postgresql://postgres:postgres@localhost:5432/biene", NoTls)
    {
        Ok(client) => client,
        Err(_e) => panic!("{}", _e), // database out of reach -> i think it's ok to panic :-)
    };
    return client;
}

// init struct "Volk" for the database query

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

// function to query data from the table "Volk"

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
        Err(_e) => return default_vk, // return a default struct
    };
    let volk = Volk::from(row);
    return volk; // return a strukt
}

// init struct "Durchsicht" for the database query

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
    pub memo: String,
}

impl From<Row> for Durchsicht {
    fn from(row: Row) -> Self {
        Self {
            id: row.get("id"),
            datum: match row.get("datum") {
                Some(val) => val,
                None => "".to_string(),
            },
            volk: row.get("volk"),
            koenigin: match row.get("koenigin") {
                Some(val) => val,
                None => false,
            },
            stifte: match row.get("stifte") {
                Some(val) => val,
                None => false,
            },
            offene: match row.get("offene") {
                Some(val) => val,
                None => false,
            },
            verdeckelte: match row.get("verdeckelte") {
                Some(val) => val,
                None => false,
            },
            weiselzelle: match row.get("weiselzelle") {
                Some(val) => val,
                None => false,
            },
            spielnaepfe: match row.get("spielnaepfe") {
                Some(val) => val,
                None => false,
            },
            sanftmut: match row.get("sanftmut") {
                Some(val) => val,
                None => 0,
            },
            volksstaerke: match row.get("volksstaerke") {
                Some(val) => val,
                None => 0,
            },
            anz_brutwaben: match row.get("anz_brutwaben") {
                Some(val) => val,
                None => 0,
            },
            memo: match row.get("memo") {
                Some(val) => val,
                None => "".to_string(),
            },
        }
    }
}

// function to query data from the table "volk"

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
        memo: "".to_string(),
    };
    let row = match client.query_one(sql, &[]) {
        Ok(row) => row,
        Err(_e) => return default_ds, // return a default strukt
    };
    let durchsicht = Durchsicht::from(row);
    return durchsicht; // return a struct
}

// function for executing a sql script against the database "biene"
// returns 0 in case of an error happening, otherwise the lines affected
// i hope this error handling and returning are clever :-)

pub fn db_execute(sql: &str, mut client: Client) -> u64 {
    // count of executed lines  in the database table
    let _lines = match client.execute(sql, &[]) {
        Ok(_lines) => return _lines,
        Err(..) => return 0,
    };
}

// function to query multiple lines of data from the table "Volk"

pub fn volk_fetchall(sql: &str, mut client: Client) -> Vec<Volk> {
    let mut default_vkv = Vec::new();
    default_vkv.push(Volk {
        id: 0,
        volk: "".to_string(),
        nummer: 0,
        koenigin: "".to_string(),
        erstellt: "".to_string(),
        aufgeloest: "".to_string(),
        typ: "".to_string(),
        raehmchenmass: "".to_string(),
        stand: "".to_string(),
    });
    let rows = match client.query(sql, &[]) {
        Ok(rows) => rows,
        Err(_e) => return default_vkv, // return a default vec struct
    };
    let mut volk_vec = Vec::new();
    for row in rows {
        volk_vec.push(Volk::from(row))
    }
    return volk_vec; // return a vec struct
}

// function to query multiple lines of data from the table "Durchsicht"

pub fn durchsicht_fetchall(sql: &str, mut client: Client) -> Vec<Durchsicht> {
    let mut default_dsv = Vec::new();
    default_dsv.push(Durchsicht {
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
        memo: "".to_string(),
    });
    let rows = match client.query(sql, &[]) {
        Ok(rows) => rows,
        Err(_e) => return default_dsv, // return a default vec struct
    };
    let mut durchsicht_vec: Vec<Durchsicht> = Vec::new();
    for row in rows {
        durchsicht_vec.push(Durchsicht::from(row))
    }
    return durchsicht_vec; // return a vec struct
}