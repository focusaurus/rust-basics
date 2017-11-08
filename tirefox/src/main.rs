extern crate rand;
extern crate shuffled_iter;
mod scratch;
use shuffled_iter::ShuffledIterGen;
use std::error::Error;
use std::fs;
use std::io;
use std::io::ErrorKind;
use std::io::prelude::*;
use std::io::Write;

const HOW_MANY: usize = 50;

fn bail(err: std::io::Error) {
    let message = match err.kind() {
        ErrorKind::NotFound => "Words dictionary file not found",
        _ => err.description(),
    };
    write!(&mut io::stderr(), "{}\n", message);
    std::process::exit(10);
}
fn is_short(word: &String) -> bool {
    word.len() < 7
}

fn main() {
    // scratch::scratch();
    let words_file = fs::File::open("/usr/share/dict/words")
        .map_err(bail)
        .unwrap();
    let words_reader = io::BufReader::new(words_file);
    let short_word_count = words_reader
        .lines()
        .map(|result| result.map_err(bail).unwrap())
        .filter(is_short)
        .count();
    // println!("{:?}", short_word_count);
    let mut chaos = rand::thread_rng();
    let mut indices: Vec<usize> = chaos
        .iter_shuffled(0usize..short_word_count as usize)
        .take(HOW_MANY * 2)
        .collect();
    indices.sort();
    // println!("indices: {:?}", indices);
    // todo DRY up these iterator chains
    let words_file2 = fs::File::open("/usr/share/dict/words").unwrap();
    // words_file.seek(io::SeekFrom::Start(0));
    let mut short_word_iter = io::BufReader::new(&words_file2)
        .lines()
        .map(|result| result.map_err(bail).unwrap())
        .filter(is_short);
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
