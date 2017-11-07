// todo generate a pair of random integers
// todo try shuffled_iter
extern crate shuffled_iter;
mod scratch;

// use random::Source;
use std::fs;
use std::io;
use std::io::prelude::*;

fn main() {
    scratch::scratch();
    // let mut source = random::default().seed([48654684, 7686898]);
    // let mut index = source.read::<usize>();
    let words_file = fs::File::open("/usr/share/dict/words").unwrap();
    let words_reader = io::BufReader::new(words_file);
    // let word = words_reader
    //     .lines()
    //     .map(|result| result.unwrap())
    //     .filter(|line| line.len() < 8)
    //     .nth(index).unwrap();
    // println!("{}", word);
    // let mut count = 0;
    //
    // for word in words_reader
    //         .lines()
    //         .map(|result| result.unwrap())
    //         .filter(|line| line.len() < 8) {
    //     count += 1;
    //     println!("{}", word);
    // }
    // println!("words: {}", count);
}
// todo use .nth() to get some random ones
