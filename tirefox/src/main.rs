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
    let message = match err.kind() {
        ErrorKind::NotFound => "Words dictionary file not found",
        _ => err.description(),
    };
    write!(&mut io::stderr(), "{}\n", message).unwrap();
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

// String
// &str    // string slice
// &String

// Vec<u8>
// &[u8]
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

fn main() {
    // count total words (one per line) in the file
    let words_file = fs::File::open(WORDS_PATH).map_err(bail).unwrap();
    let words_reader = io::BufReader::new(&words_file);
    let short_word_count = short_lines(words_reader).count();

    // Prepare the iterator of the words themselves
    let mut reader = io::BufReader::new(&words_file);
    reader.seek(io::SeekFrom::Start(0)).unwrap();
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
                       .collect::<Vec<_>>();
    for pair in results.chunks(2) {
        println!("{}{}", pair[0], pair[1]);
    }
}
