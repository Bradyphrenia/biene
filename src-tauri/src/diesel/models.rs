use diesel::prelude::*;
use diesel::sql_types::SmallInt;
use serde::{ser::SerializeStruct, Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
pub enum DatabaseTable {
    Durchsicht,
}

// #[derive(Queryable, Selectable)]
// #[diesel(table_name = crate::diesel::schema::posts)]
// #[diesel(check_for_backend(diesel::pg::Pg))]
// pub struct Post {
//     pub id: i32,
//     pub title: String,
//     pub body: String,
//     pub published: bool,
// }

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = crate::diesel::schema::durchsicht)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Durchsicht {
    pub id: i32,
    // #[serde(with = "PgDateDef")]
    pub datum: Option<diesel::pg::data_types::PgDate>,
    pub volk: Option<String>,
    pub koenigin: Option<bool>,
    pub stifte: Option<bool>,
    pub offene: Option<bool>,
    pub verdeckelte: Option<bool>,
    pub weiselzelle: Option<bool>,
    pub spielnaepfe: Option<bool>,
    pub sanftmut: Option<i16>,
    pub volksstaerke: Option<i16>,
    pub anz_brutwaben: Option<i16>,
}

impl serde::Serialize for Durchsicht {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        println!("{:?}, {:?}", &self.datum, &self.koenigin);
        let mut s = serializer.serialize_struct("Durchsicht", 1)?;
        s.serialize_field("id", &self.id)?;
        // s.serialize_field("datum", &self.datum)?;
        s.serialize_field("volk", &self.volk)?;
        s.serialize_field("koenigin", &self.koenigin)?;
        s.serialize_field("stifte", &self.stifte)?;
        s.serialize_field("offene", &self.offene)?;
        s.serialize_field("verdeckelte", &self.verdeckelte)?;
        s.serialize_field("weiselzelle", &self.weiselzelle)?;
        s.serialize_field("spielnaepfe", &self.spielnaepfe)?;
        s.serialize_field("sanftmut", &self.sanftmut)?;
        s.serialize_field("volksstaerke", &self.volksstaerke)?;
        s.serialize_field("anz_brutwaben", &self.anz_brutwaben)?;
        s.end()
    }
}

// #[derive(Serialize, Deserialize)]
// #[serde(remote = "PgDate")]
// struct PgDateDef ( i32 );

// impl serde::Serialize for diesel::pg::data_types::PgDate {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: serde::Serializer {
//         serde_json::to_string(&self)
//     }
// }
