extern crate rand;
extern crate structopt;
#[macro_use(StructOpt)]
extern crate structopt_derive;

use rand::Rng;
use std::{fs, io};
use std::io::ErrorKind;
use std::io::prelude::*;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "tirefox", about = "Generate ellisions of random short words")]
struct Opt {

    #[structopt(short = "w", long = "words", parse(from_os_str), default_value="/usr/share/dict/words")]
    words_path: PathBuf,

    /// Number of words
    #[structopt(short = "c", long = "count", default_value = "20")]
    count: usize,
}

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
    let opt = Opt::from_args();
    let words_file = fs::File::open(opt.words_path)?;
    let words_reader = io::BufReader::new(&words_file);
    let how_many = opt.count * 2;
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
