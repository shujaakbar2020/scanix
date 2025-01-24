use std::fs::File;
use std::io::{BufReader, Result};
use std::io::prelude::*;
use regex::Regex;
use clap::{App,Arg};
use std::collections::HashMap;

struct BoyerMoore {
    pattern: Vec<u8>,
    bad_char_table: [i32; 256],
}

impl BoyerMoore {
    pub fn new(pattern: &[u8]) -> Self {
        let mut bad_char_table = [-1; 256];
        for (i, &byte) in pattern.iter().enumerate() {
            bad_char_table[byte as usize] = i as i32;
        }
        BoyerMoore {
            pattern: pattern.to_vec(),
            bad_char_table,
        }
    }

    pub fn search(&self, text: &[u8]) -> Vec<usize> {
        let mut result = vec![];
        let n = text.len();
        let m = self.pattern.len();
        if m == 0 || n < m {
            return result;
        }

        let mut i = 0; // current start index in text
        while i <= n - m {
            let mut j = (m - 1) as isize;
            // Compare from the end of the pattern backwards
            while j >= 0 && text[i + j as usize] == self.pattern[j as usize] {
                j -= 1;
            }
            if j < 0 {
                // Found a match at position i
                result.push(i);
                // Shift based on the next character after the current match
                if i + m >= n {
                    break;
                }
                let c = text[i + m] as usize;
                let shift = m as i32 - self.bad_char_table[c];
                i += std::cmp::max(shift, 1) as usize;
            } else {
                // Mismatch occurred, shift based on bad character rule
                let j_usize = j as usize;
                let c = text[i + j_usize] as usize;
                let bc_shift = j as i32 - self.bad_char_table[c];
                let shift = if bc_shift <= 0 { 1 } else { bc_shift as usize };
                i += shift;
            }
        }

        result
    }
}

fn search_file_with_regex() {
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

    println!("{}, {}", args.value_of("pattern").unwrap(), args.value_of("file").unwrap());
    let pattern = args.value_of("pattern").unwrap();
    let file_path = args.value_of("file").unwrap();
    let re = Regex::new(pattern).unwrap();

    let f: File = File::open(file_path).unwrap();
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
    // search_file_with_regex();

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
    let f_ = File::open(file_path).unwrap();
    let reader = BufReader::new(f_);

    for line_ in reader.lines() {
        let text = line_.unwrap();
        let bm = BoyerMoore::new(pattern.as_bytes());
        let matches = bm.search(text.as_bytes());
        if matches.len() != 0 {
            println!("{}", text);
        }
    }
}
