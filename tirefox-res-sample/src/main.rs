extern crate rand;
use std::{fs, io};
use std::io::prelude::*;
use rand::Rng;

const HOW_MANY: usize = 20;
const WORDS_PATH: &str = "/usr/share/dict/words";

fn suitable(word: &str) -> bool {
    word.len() > 2 && word.len() < 8 && word.chars().nth(0).map(char::is_lowercase).unwrap_or(false)
}

fn tirefox() -> io::Result<Vec<String>> {
    let words_file = fs::File::open(WORDS_PATH)?;
    let words_reader = io::BufReader::new(&words_file);
    let mut sample = Vec::with_capacity(HOW_MANY);
    for (index, line_result) in words_reader.lines().enumerate() {
        let word = line_result?;
        if !suitable(&word) {
            continue;
        }
        if sample.len() < HOW_MANY {
            // sample is sparse, always fill it initially
            sample.push(word);
        } else {
            let evictee_M = rand::thread_rng().gen_range(0, index + 1);
            if evictee_M < HOW_MANY {
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
        },
        Err(e) => eprintln!("{}",e),
    }
}
