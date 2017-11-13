#[macro_use]
extern crate clap;
extern crate rand;
use clap::{Arg, App};
use rand::Rng;
use std::{fs, io};
use std::io::prelude::*;

const WORDS_PATH: &str = "/usr/share/dict/words";

fn suitable(word: &str) -> bool {
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
    let words_reader = io::BufReader::new(&words_file);
    let mut how_many = value_t!(matches, "count", usize).unwrap_or(10) * 2;
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
            let evictee_M = rand::thread_rng().gen_range(0, index + 1);
            if evictee_M < how_many {
                sample[evictee_M] = word;
            }
        }
    }
    Ok(sample)
}

fn main() {
    match tirefox() {
        Ok(sample) => {
            for pair in sample.chunks(2) {
                println!("{}{}", pair[0], pair[1]);
            }
        }
        Err(e) => eprintln!("{}", e),
    }
}
