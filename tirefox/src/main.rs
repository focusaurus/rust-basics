use std::fs;
use std::io;
use std::io::prelude::*;

fn main() {
    let words_file = fs::File::open("/usr/share/dict/words").unwrap();
    let words_reader = io::BufReader::new(words_file);
    let mut count = 0;

    for word in words_reader
            .lines()
            .map(|result| result.unwrap())
            .filter(|line| line.len() < 8) {
        count += 1;
        println!("{}", word);
    }
    println!("words: {}", count);
}
// todo use .nth() to get some random ones
