mod database_mod;

use crate::database_mod::init_db;
use crate::database_mod::volk_fetchone;
use crate::database_mod::{db_execute, durchsicht_fetchone};

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
    );
    let test1 = db_execute("INSERT INTO durchsicht ", init_db()); // unvollständiges Script
    println!("{} Zeile(n) eingefügt.", test1);
    let test2 = db_execute("INSERT INTO durchsicht (datum, volk, koenigin, stifte, offene, verdeckelte, weiselzelle, spielnaepfe, sanftmut, volksstaerke, anz_brutwaben) VALUES ('2023-03-25', 'Volk 01', TRUE, TRUE, TRUE, TRUE, FALSE, FALSE, 5, 4, 4);", init_db()); // vollständiges Script
    println!("{} Zeile(n) eingefügt.", test2);
}
