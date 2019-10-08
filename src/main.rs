use std::fs::File;
use std::io::{self, BufRead, Write};
use std::path::Path;

use fuzzy_matcher::skim::fuzzy_match;
use rand::Rng;

// exponential growth, doubles every DOUBLE_BY steps
const DOUBLE_BY: f64 = 10.0;
fn factor() -> f64 {
    ((2.0_f64).ln() / DOUBLE_BY).exp()
}

#[derive(Debug, Clone)]
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
        println!(
            "(p)en, (i)nk, (mp) manual pen selection, (mi) manual ink selection, (s)ave, (A)bort"
        );
        print!("=> ");
        io::stdout().flush().unwrap();
        let mut answer = String::new();
        io::stdin()
            .read_line(&mut answer)
            .expect("Failed to read line");
        match answer.trim() {
            "p" => selected_pen = weighted_random_selection(&records_pens),
            "i" => selected_ink = weighted_random_selection(&records_inks),
            "mp" => selected_pen = manual_selection(&records_pens),
            "mi" => selected_ink = manual_selection(&records_inks),
            "s" => break,
            _ => {
                selected_pen = None;
                selected_ink = None;
                break;
            }
        }
        println!();
    }

    let records_pens = update_records(records_pens.to_vec(), selected_pen);
    let records_inks = update_records(records_inks.to_vec(), selected_ink);

    write(records_pens, &filename_pens);
    write(records_inks, &filename_inks);
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

fn manual_selection(records: &Vec<Record>) -> Option<&Record> {
    print!("Search string: ");
    io::stdout().flush().unwrap();
    let mut answer = String::new();
    io::stdin()
        .read_line(&mut answer)
        .expect("Failed to read line");

    let mut records: Vec<(&Record, i64)> = records
        .iter()
        .filter_map(|record| {
            fuzzy_match(&record.name, &answer.trim()).and_then(|score| Some((record, score)))
        })
        .collect();
    if records.is_empty() {
        println!("No results");
        return None;
    }
    records.sort_by_key(|(_, score)| *score);
    records.reverse();
    let mut index = 1;
    for (r, _) in records.iter() {
        println!("({}) {}", index, r.name);
        index += 1;
    }

    print!("Selection: ");
    io::stdout().flush().unwrap();
    answer = String::new();
    io::stdin()
        .read_line(&mut answer)
        .expect("Failed to read line");
    match answer.trim().parse::<usize>().ok() {
        Some(v) => records.get(v - 1).and_then(|(record, _)| Some(*record)),
        None => None,
    }
}

fn update_records(records: Vec<Record>, selection: Option<&Record>) -> Vec<Record> {
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

fn read(filename: &std::path::Path) -> Vec<Record> {
    let mut result: Vec<Record> = vec![];
    let file = File::open(filename).unwrap();
    let file_reader = io::BufReader::new(file);
    for line in file_reader.lines().map(|l| l.unwrap()) {
        let fields: Vec<&str> = line.trim().split(",").collect();
        result.push(Record {
            name: String::from(fields[0]),
            p: fields[1].parse::<f64>().unwrap(),
        });
    }
    result
}

fn write(records: Vec<Record>, filename: &std::path::Path) {
    let file = File::create(filename).unwrap();
    let mut file_writer = io::BufWriter::new(file);
    for record in records {
        let r = write!(file_writer, "{},{}\n", record.name, record.p);
        r.expect(&format!("Can't write {:?}", record));
    }
    file_writer.flush().unwrap();
}
