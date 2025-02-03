use std::fs::File;
use std::io::{BufReader};
use std::io::prelude::*;
use regex::Regex;
use clap::{App,Arg};

const PRIME: u64 = 101;

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

#[allow(dead_code)]
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

fn rabin_karp_hash(text: &[u8], len: usize) -> u64 {
    let mut hash = 0;
    for &c in text.iter().take(len) {
        hash = (hash * 256 + c as u64) % PRIME;
    }
    hash
}

fn rabin_karp_rolling_hash(prev_hash: u64, old_char: u8, new_char: u8, m: usize) -> u64 {
    let old_hash = (prev_hash + PRIME - (old_char as u64 * 256u64.pow((m - 1) as u32)) % PRIME) % PRIME;
    (old_hash * 256 + new_char as u64) % PRIME
}

fn highlight_text(text: &str, pattern: &str, matches: &[usize]) -> String{
    let mut highlighted_text = String::new();
    let mut last_idx = 0;
    for &start in matches {
        highlighted_text.push_str(&text[last_idx..start]);
        highlighted_text.push_str("\x1b[31m");
        highlighted_text.push_str(&text[start..start + pattern.len()]);
        highlighted_text.push_str("\x1b[0m");
        last_idx = start + pattern.len();
    }
    highlighted_text.push_str(&text[last_idx..]);
    highlighted_text
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

    let pattern_bytes = pattern.as_bytes();
    let m = pattern_bytes.len();
    let pattern_hash = rabin_karp_hash(pattern_bytes, m);


    for line_ in reader.lines() {
        let text = line_.unwrap();
        let text_bytes = text.as_bytes();
        let n = text_bytes.len();
        if n < m {
            continue;
        }
        let mut text_hash = rabin_karp_hash(text_bytes, m);
        let mut candidate_positions = vec![];

        if text_hash == pattern_hash && &text_bytes[0..m] == pattern_bytes {
            candidate_positions.push(0);
        }

        for i in 1..= n - m {
            text_hash = rabin_karp_rolling_hash(text_hash, text_bytes[i - 1], text_bytes[i + m - 1], m);
            if text_hash == pattern_hash && &text_bytes[i..i + m] == pattern_bytes {
                candidate_positions.push(i);
            }
        }

        if !candidate_positions.is_empty() {
            let bm = BoyerMoore::new(pattern_bytes);
            let matches = bm.search(text_bytes);
            if !matches.is_empty() {
                let highlighted = highlight_text(&text, pattern, &matches);
                println!("{}", highlighted);
            }
        }
    }
}
