use std::io;
use std::io::Write;
use std::path::Path;

use rand::Rng;
use serde::{Deserialize, Serialize};

// exponential growth, doubles every DOUBLE_BY steps
const DOUBLE_BY: f64 = 10.0;
fn factor() -> f64 {
    ((2.0_f64).ln() / DOUBLE_BY).exp()
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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

    let mut selected_pen: Option<&Record> = None;
    let mut selected_ink: Option<&Record> = None;

    loop {
        println!("Pen: {}", selected_pen.map_or("None", |r| &r.name));
        println!("Ink: {}", selected_ink.map_or("None", |r| &r.name));
        println!("(p)en, (i)nk, (s)ave, (A)bort");
        io::stdout().flush().unwrap();
        let mut answer = String::new();
        io::stdin()
            .read_line(&mut answer)
            .expect("Failed to read line");
        match answer.trim() {
            "p" => {
                selected_pen = weighted_random_selection(&records_pens);
            }
            "i" => {
                selected_ink = weighted_random_selection(&records_inks);
            }
            "s" => break,
            _ => {
                selected_pen = None;
                selected_ink = None;
                break;
            }
        }
        println!("");
    }

    let records_pens = update_records(records_pens.to_vec(), &selected_pen);
    let records_inks = update_records(records_inks.to_vec(), &selected_ink);

    write(records_pens, &filename_pens);
    write(records_inks, &filename_inks);
}

fn update_records(records: Vec<Record>, selection: &Option<&Record>) -> Vec<Record> {
    match selection {
        None => records,
        Some(selection) => {
            let records = records
                .iter()
                .map(|x| Record {
                    p: if x.name == selection.name {
                        1.0
                    } else {
                        x.p * factor()
                    },
                    name: x.name.clone(),
                })
                .collect();
            records
        }
    }
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
