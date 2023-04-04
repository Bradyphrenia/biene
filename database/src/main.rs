mod database;
use database::dbase::{durchsicht_fetchall, durchsicht_fetchone, volk_fetchall, volk_fetchone};

fn main() {
    /*
    let test0 = db_execute("ALTER TABLE durchsicht ADD COLUMN memo varchar;", init_db()); // Einfügen des Memofeldes
    println!("{} Zeile(n) eingefügt.", test0);
     */
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
}
