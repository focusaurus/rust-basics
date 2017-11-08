// todo generate a pair of random integers
// todo try shuffled_iter
extern crate rand;
extern crate shuffled_iter;
mod scratch;
use shuffled_iter::ShuffledIterGen;

use std::fs;
use std::io;
use std::io::prelude::*;
use std::io::ErrorKind;
use std::io::Write;
use std::error::Error;

const HOW_MANY: usize = 50;

fn main() {
    // scratch::scratch();
    let words_file = fs::File::open("/usr/share/dict/wordsNOPE")
        .map_err(|err| {
            let message = match err.kind() {
                ErrorKind::NotFound => "Words dictionary file not found",
                _ => err.description(),
            };
            io::stderr().write_fmt(format_args!("{}\n", message));
            std::process::exit(10);
            // if let Some(os_err) = err.raw_os_error() {
            //     let message = match os_err {
            //         2 => "Words dictionary file not found",
            //         _ => err.description(),
            //     };
            //     println!("Error {:?}", err);
            //     std::process::exit(10);
            //
            //
            //     println!("raw OS error: {:?}", raw_os_err);
            // } else {
            //     println!("Not an OS error");
            // }
        })
        .unwrap();
    // if words_result.is_err() {
    // }
    // if let Err(err) = words_result {
    let words_reader = io::BufReader::new(words_file);
    let short_word_count = words_reader
        .lines()
        .map(|result| result.unwrap())
        .filter(|line| line.len() < 7)
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
        .map(|result| result.unwrap())
        .filter(|line| line.len() < 7);
    let mut last = 0;
    let results = &indices
                       .iter()
                       .map(|&index| {
                                let word = short_word_iter.nth(index - last - 1);
                                last = index;
                                word.unwrap()
                            })
                       .collect::<Vec<_>>();
    for pair in results.chunks(2) {
        println!("{}{}", pair[0], pair[1]);
    }
}
