// todo generate a pair of random integers
// todo try shuffled_iter
extern crate rand;
extern crate shuffled_iter;
mod scratch;
use shuffled_iter::ShuffledIterGen;

use std::fs;
use std::io;
use std::io::prelude::*;

fn main() {
    // scratch::scratch();
    // let mut source = random::default().seed([48654684, 7686898]);
    // let mut index = source.read::<usize>();
    let mut words_file = fs::File::open("/usr/share/dict/words").unwrap();
    let how_many = 4;
    let mut words_reader = io::BufReader::new(&words_file);
    let short_word_count = words_reader
        .lines()
        .map(|result| result.unwrap())
        .filter(|line| line.len() < 7)
        .count();
    // println!("{:?}", short_word_count);
    let mut chaos = rand::thread_rng();
    let mut indices: Vec<usize> = chaos
        .iter_shuffled(0usize..short_word_count as usize)
        .take(how_many * 2)
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
                                let word = short_word_iter.nth(index - last);
                                last = index;
                                word.unwrap()
                            })
                       .collect::<Vec<_>>();
    let mut first: Option<&str> = None;
    for pair in results.chunks(2) {
        println!("{}{}", pair[0], pair[1]);
    }
}
