extern crate csv;
extern crate rand;

use std::path::Path;
use std::io;
use std::io::Write;
use rand::Rng;

const DOUBLE_BY: f64 = 10.0;

struct Record {
    name: String,
    p:    f64,
}


fn main() {
	let dirname = Path::new("./");

    let filename_pens = dirname.join("pens.csv");
    let filename_inks = dirname.join("inks.csv");

	let records_pens = read(&filename_pens);
	let records_inks = read(&filename_inks);

    let mut select_pen:Option<&Record>;
    let mut select_ink:Option<&Record>;

    loop {
        select_pen = weighted_random_selection(&records_pens);
        select_ink = weighted_random_selection(&records_inks);

        println!("Stift: {}",select_pen.unwrap().name);
        println!("Tinte: {}",select_ink.unwrap().name);

        print!("Zufrieden? ");
        io::stdout().flush();
        let mut answer = String::new();
        io::stdin().read_line(&mut answer)
                   .expect("Failed to read line");
        match answer.trim() {
            "y" => break,
            _   => continue,
        }
    }

    let records_pens = update_records(&records_pens,&select_pen);
    let records_inks = update_records(&records_inks,&select_ink);

    write(records_pens,&filename_pens);
    write(records_inks,&filename_inks);
}


fn update_records(records:&Vec<Record>,selection:&Option<&Record>) -> Vec<Record> {
    let records = records
                  .iter()
                  .map(|x| Record {p: x.p*factor(), name: x.name.clone()})
                  .map(|x| if x.name == selection.unwrap().name {
                               Record {p:1.0,name:x.name.clone()}
                           } else {x})
                  .collect();
    records
}


fn factor() -> f64 {
    ((2.0_f64).ln() / DOUBLE_BY).exp()
}


fn weighted_random_selection(records:&Vec<Record>) -> Option<&Record> {
	let sum = records.iter()
                     .fold(0.0, |acc, x| acc + x.p);
    let mut randnum = rand::thread_rng().gen_range(0, sum.floor() as i64) as f64;
	for record in records.iter() {
        if randnum <= record.p {return Some(record);}
        else {randnum -= record.p;}
	}
    None
}


fn read(filename:&std::path::Path) -> Vec<Record> {
    let mut rdr = match csv::Reader::from_file(filename) {
        Ok(w) => w.has_headers(false),
        Err(_) => panic!("Panic at the reader"),
    };
    let mut r: Vec<Record> = vec![];
	for row in rdr.decode() {
		let row = row.unwrap();
        let (n, p): (String, f64) = row;
		r.push(Record {name: n, p: p});
    }
	r
}


fn write(records:Vec<Record>,filename:&std::path::Path) {
    let mut wtr = match csv::Writer::from_file(filename) {
        Ok(w) => w,
        Err(_) => panic!("Panic at the writer"),
    };
    for record in records.into_iter() {
		let record:(String,f64) = (record.name, record.p);
        println!("{:?}",record);
        let result = wtr.encode(record);
        assert!(result.is_ok());
    }
}
