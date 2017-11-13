#[macro_use]
extern crate clap;
extern crate rand;
use clap::{Arg, App};
use rand::Rng;
use std::{fs, io};
use std::io::ErrorKind;
use std::io::prelude::*;

const WORDS_PATH: &str = "/usr/share/dict/words";

fn suitable(word: &str) -> bool {
    // not too short
    // not too long
    // not a capitalized proper name
    word.len() > 2 && word.len() < 8 &&
    word.chars()
        .nth(0)
        .map(char::is_lowercase)
        .unwrap_or(false)
}

fn tirefox() -> io::Result<Vec<String>> {
    let matches = App::new("tirefox")
        .version(crate_version!())
        .about("Generate ellisions of random short words")
        .arg(Arg::with_name("words_path")
                 .short("w")
                 .long("words")
                 .takes_value(true)
                 // .with_default("/usr/share/dict/words")
                 .help("Word dictionary path. One word per line."))
        .arg(Arg::with_name("count")
                 .short("c")
                 .long("count")
                 .takes_value(true)
                 .help("How many words to generate"))
        .get_matches();
    let words_file = fs::File::open(matches.value_of("words_path").unwrap_or(WORDS_PATH))?;
    // words_file.map_err(|e| io::Error::new(e.kind(), format!("Unable to read words file: {}", e)))?;
    let words_reader = io::BufReader::new(&words_file);
    let how_many = value_t!(matches, "count", usize)
        .map_err(|e| e.exit())
        .unwrap() * 2;
    let mut sample = Vec::with_capacity(how_many);
    for (index, line_result) in words_reader.lines().enumerate() {
        let word = line_result?;
        if !suitable(&word) {
            continue;
        }
        if sample.len() < how_many {
            // sample is sparse, always fill it initially
            sample.push(word);
        } else {
            let index_to_replace = rand::thread_rng().gen_range(0, index + 1);
            if index_to_replace < how_many {
                sample[index_to_replace] = word;
            }
        }
    }
    Ok(sample)
}

fn main() {
    match tirefox() {
        Err(error) => {
            let message = match error.kind() {
                ErrorKind::NotFound => "Words file not found",
                ErrorKind::PermissionDenied => "Words file not readable",
                _ => "Unexpected IO error reading words file",

            };
            eprintln!("{}", message);
            std::process::exit(10);
        }
        Ok(sample) => {
            for pair in sample.chunks(2) {
                println!("{}{}", pair[0], pair[1]);
            }
        }
    }
}
