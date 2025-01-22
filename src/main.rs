use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use regex::Regex;
use clap::{App,Arg};

fn search_file() {
    let args = App::new("Scanix")
        .version(env!("CARGO_PKG_VERSION"))
        .about("searches for patterns")
        .arg(Arg::with_name("pattern")
            .help("The pattern to search for")
            .takes_value(true)
            .required(true))
        .arg(Arg::with_name("file")
            .help("The file to search in")
            .takes_value(true)
            .required(true))
        .get_matches();
    let pattern = args.value_of("pattern").unwrap();
    let file_path = args.value_of("file").unwrap();
    let re = Regex::new(pattern).unwrap();

    let f = File::open(file_path).unwrap();
    let reader = BufReader::new(f);

    for line_ in reader.lines() {
        let line = line_.unwrap();
        match re.find(&line) {
            Some(_) => println!("{}", line),
            None => (),
        }
    }
}

fn main() {
    search_file();
}
