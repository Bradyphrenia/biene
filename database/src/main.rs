mod database_mod;

use crate::database_mod::init_db;
use crate::database_mod::{db_execute, durchsicht_fetchall, durchsicht_fetchone};
use crate::database_mod::{volk_fetchall, volk_fetchone};

fn main() {
    let test0 = db_execute(
        "ALTER TABLE durchsicht ADD COLUMN memo varchar();",
        init_db(),
    ); // Einfügen des Memofeldes
    println!("{} Zeile(n) eingefügt.", test0);
    let test3 = volk_fetchone("SELECT id, volk, nummer, koenigin, erstellt::varchar, aufgeloest::varchar, typ, raehmchenmass, stand FROM volk WHERE volk = 'Volk 01';", init_db());
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
    let test4 = durchsicht_fetchone("SELECT id, datum::varchar, volk, koenigin, stifte, offene, verdeckelte, weiselzelle, spielnaepfe, sanftmut, volksstaerke, anz_brutwaben, memo FROM durchsicht WHERE id = 26;", init_db());
    println!(
        "{} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} ",
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
        test4.memo,
    );
    /*
    let test1 = db_execute("INSERT INTO durchsicht ...", init_db()); // unvollständiges Script
    println!("{} Zeile(n) eingefügt.", test1);
    let test2 = db_execute("INSERT INTO durchsicht (datum, volk, koenigin, stifte, offene, verdeckelte, weiselzelle, spielnaepfe, sanftmut, volksstaerke, anz_brutwaben, memo) VALUES ('2023-03-25', 'Volk 01', TRUE, TRUE, TRUE, TRUE, FALSE, FALSE, 5, 4, 4, 'Das ist ein Test!!!');", init_db()); // vollständiges Script
    println!("{} Zeile(n) eingefügt.", test2);
     */
    for i in 0..100 {
        println!("{}-er Durchlauf: ", i);
        let test5 = durchsicht_fetchall("SELECT id, datum::varchar, volk, koenigin, stifte, offene, verdeckelte, weiselzelle, spielnaepfe, sanftmut, volksstaerke, anz_brutwaben, memo FROM durchsicht;", init_db());
        for x in test5 {
            println!(
                "{} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} ",
                x.id,
                x.datum,
                x.volk,
                x.koenigin,
                x.stifte,
                x.offene,
                x.verdeckelte,
                x.weiselzelle,
                x.spielnaepfe,
                x.sanftmut,
                x.volksstaerke,
                x.anz_brutwaben,
                x.memo,
            );
        }
        let test6 = volk_fetchall("SELECT id, volk, nummer, koenigin, erstellt::varchar, aufgeloest::varchar, typ, raehmchenmass, stand FROM volk ;", init_db());
        for x in test6 {
            println!(
                "{} | {} | {} | {} | {} | {} | {} | {} | {} ",
                x.id,
                x.volk,
                x.nummer,
                x.koenigin,
                x.erstellt,
                x.aufgeloest,
                x.typ,
                x.raehmchenmass,
                x.stand
            );
        }
    }
    println!("100 Durchläufe realisiert.");
    let _test7 = db_execute("SELECT setval('volk_seq', 19, true);", init_db());

    let _test8 = db_execute(
        "UPDATE durchsicht SET volk = 'Volk 11' WHERE volk = 'Volk 011';",
        init_db(),
    );
    let _test8 = db_execute(
        "UPDATE durchsicht SET volk = 'Volk 10' WHERE volk = 'Volk 010';",
        init_db(),
    );
    let _test8 = db_execute(
        "UPDATE durchsicht SET volk = 'Volk 09' WHERE volk = 'Volk 009';",
        init_db(),
    );
    let _test8 = db_execute(
        "UPDATE durchsicht SET volk = 'Volk 08' WHERE volk = 'Volk 008';",
        init_db(),
    );
    let _test8 = db_execute(
        "UPDATE durchsicht SET volk = 'Volk 07' WHERE volk = 'Volk 007';",
        init_db(),
    );
    let _test8 = db_execute(
        "UPDATE durchsicht SET volk = 'Volk 06' WHERE volk = 'Volk 006';",
        init_db(),
    );
    let _test8 = db_execute(
        "UPDATE durchsicht SET volk = 'Volk 05' WHERE volk = 'Volk 005';",
        init_db(),
    );
    let _test8 = db_execute(
        "UPDATE durchsicht SET volk = 'Volk 04' WHERE volk = 'Volk 004';",
        init_db(),
    );
    let _test8 = db_execute(
        "UPDATE durchsicht SET volk = 'Volk 03' WHERE volk = 'Volk 003';",
        init_db(),
    );
    let _test8 = db_execute(
        "UPDATE durchsicht SET volk = 'Volk 02' WHERE volk = 'Volk 002';",
        init_db(),
    );
    let _test8 = db_execute(
        "UPDATE durchsicht SET volk = 'Volk 01' WHERE volk = 'Volk 001';",
        init_db(),
    );
    let _test8 = db_execute("SELECT setval('volk_seq', 19, true);", init_db());
    let _test8 = db_execute("SELECT setval('veterinaer_seq', 3, true);", init_db());
    let _test8 = db_execute("SELECT setval('zuechter_seq', 8, true);", init_db());
    let _test8 = db_execute("SELECT setval('durchsicht_seq', 141, true);", init_db());
    let _test8 = db_execute("SELECT setval('stand_seq', 6, true);", init_db());
}
