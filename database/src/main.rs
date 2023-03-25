mod database_mod;
use crate::database_mod::durchsicht_fetchone;
use crate::database_mod::init_db;
use crate::database_mod::volk_fetchone;

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
