use std::fs::File;
use std::io::BufReader;
use std::io::prelude::BufRead;

fn main() {
    let words_file = File::open("/usr/share/dict/words").unwrap();
    let words_reader = BufReader::new(words_file);
    let mut count = 0;

    for line in words_reader.lines() {
        count += 1;
        println!("{}", line.unwrap());
    }
    println!("words: {}", count);
}
