use std::path::Path;
use std::io;
use std::io::Write;

use rand::Rng;
use serde::{Serialize, Deserialize};

// exponential growth, doubles every DOUBLE_BY steps
const DOUBLE_BY: f64 = 10.0;
fn factor() -> f64 {
    ((2.0_f64).ln() / DOUBLE_BY).exp()
}


#[derive(Serialize, Deserialize, Debug)]
struct Record {
    name: String,
    p: f64,
}


fn main() {
    let dirname = Path::new("./");

    let filename_pens = dirname.join("pens.csv");
    let filename_inks = dirname.join("inks.csv");

    let records_pens = read(&filename_pens);
    let records_inks = read(&filename_inks);

    let mut select_pen = weighted_random_selection(&records_pens);
    let mut select_ink = weighted_random_selection(&records_inks);

    loop {
        println!("Stift: {}", select_pen.unwrap().name);
        println!("Tinte: {}", select_ink.unwrap().name);
        print!("Zufrieden? ");

        io::stdout().flush().unwrap();
        let mut answer = String::new();
        io::stdin().read_line(&mut answer).expect("Failed to read line");
        match answer.trim() {
            "y" => break,
            "p" => select_pen = weighted_random_selection(&records_pens),
            "i" => select_ink = weighted_random_selection(&records_inks),
            _ => {
                select_pen = weighted_random_selection(&records_pens);
                select_ink = weighted_random_selection(&records_inks);
            }
        }
    }

    let records_pens = update_records(&records_pens, &select_pen);
    let records_inks = update_records(&records_inks, &select_ink);

    write(records_pens, &filename_pens);
    write(records_inks, &filename_inks);
}


fn update_records(records: &Vec<Record>, selection: &Option<&Record>) -> Vec<Record> {
    let records = records
        .iter()
        .map(|x| Record {
            p: if x.name == selection.unwrap().name { 1.0 }
               else { x.p * factor() },
            name: x.name.clone(),
        }).collect();
    records
}


fn weighted_random_selection(records: &Vec<Record>) -> Option<&Record> {
    let sum = records.iter().fold(0.0, |acc, x| acc + x.p);
    let mut randnum = rand::thread_rng().gen_range(0.0, sum);
    for record in records.iter() {
        if randnum <= record.p {
            return Some(record);
        } else {
            randnum -= record.p;
        }
    }
    None
}


fn read(filename: &std::path::Path) -> Vec<Record> {
    let mut rdr = csv::Reader::from_path(filename).unwrap();
    let mut r: Vec<Record> = vec![];
    for row in rdr.deserialize() {
        let record: Record = row.expect("csv record");
        r.push(record);
    }
    r
}


fn write(records: Vec<Record>, filename: &std::path::Path) {
    let mut wtr = csv::Writer::from_path(filename).unwrap();
    for record in records.into_iter() {
        let result = wtr.serialize(record);
        assert!(result.is_ok());
    }
}
