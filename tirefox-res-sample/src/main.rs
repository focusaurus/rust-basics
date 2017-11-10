/*
extern crate rand;
extern crate shuffled_iter;
use shuffled_iter::ShuffledIterGen;
use std::error::Error;
use std::fs;
use std::io;
use std::io::ErrorKind;
use std::io::prelude::*;
use std::io::Write;
use std::iter;

const HOW_MANY: usize = 50;
const WORDS_PATH: &str = "/usr/share/dict/words";

fn bail(err: std::io::Error) {
    eprintln!("{}", match err.kind() {
        ErrorKind::NotFound => "Words dictionary file not found",
        _ => err.description(),
    });
    std::process::exit(10);
}

fn is_short(word: &String) -> bool {
    word.len() < 7
}

fn bail_unwrap(result: Result<String, io::Error>) -> String {
    result.map_err(bail).unwrap()
}

fn sorted_indices(max: usize, how_many: Option<usize>) -> Vec<usize> {
    let mut chaos = rand::thread_rng();
    let mut indices: Vec<usize> = chaos
        .iter_shuffled(0usize..max as usize)
        .take(how_many.unwrap_or(50) * 2)
        .collect();
    indices.sort();
    indices
}

fn foo() -> &'static io::BufRead{
    panic!()
}

/*
fn fib<'a>(n: u64) -> &'a [u64] {
    unimplemented!()
}

fn short_lines_boxed<R>(reader: R) -> impl iter::Iterator<Item=io::Result<String>>
    where R: io::BufRead
{
    unimplemented!()
}
*/
fn short_lines<T>(reader: T)
                  -> iter::Filter<iter::Map<io::Lines<T>, fn(Result<String, io::Error>) -> String>,
                                  fn(&String) -> bool>
    where T: io::BufRead
{
    reader
        .lines()
        .map(bail_unwrap as _)
        .filter(is_short as _)
}

fn tirefox() -> io::Result<i32> {
    // count total words (one per line) in the file
    let words_file = fs::File::open(WORDS_PATH)?;
    let words_reader = io::BufReader::new(&words_file);
    let short_word_count = short_lines(words_reader).count();

    // Prepare the iterator of the words themselves
    let mut reader = io::BufReader::new(&words_file);
    reader.seek(io::SeekFrom::Start(0))?;
    let mut short_word_iter = short_lines(reader);

    let indices = sorted_indices(short_word_count, Some(HOW_MANY));
    let mut last = 0;
    let results = &indices
                       .iter()
                       .map(|&index| {
                                let word = short_word_iter.nth(index - last - 1).unwrap();
                                last = index;
                                word
                            })
                        // .ok_or(io::Error::new(io::ErrorKind::Other, "index error"))?
                       .collect::<Vec<_>>();
    for pair in results.chunks(2) {
        println!("{}{}", pair[0], pair[1]);
    }
    Ok(0)
}

fn main() {
    tirefox().map_err(bail);
}
*/
extern crate rand;
use std::{fs, io};
use std::io::prelude::*;
use rand::Rng;

const HOW_MANY: usize = 20;
const WORDS_PATH: &str = "/usr/share/dict/words";
// const WORDS_PATH: &str = "/tmp/words";

fn suitable(word: &str) -> bool {
    word.len() > 2 && word.len() < 8 && word.chars().nth(0).map(char::is_lowercase).unwrap_or(false)
}

fn tirefox() -> io::Result<Vec<String>> {
    let words_file = fs::File::open(WORDS_PATH)?;
    let words_reader = io::BufReader::new(&words_file);
    let mut sample = Vec::with_capacity(HOW_MANY);
    // let mut sample_size_m = HOW_MANY;
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
