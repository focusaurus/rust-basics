// use std::io::prelude::*;
// use std::io::Read;
// use std::ops::Try;
use std::fs::File;
use std::io::BufReader;

fn main() {
    let words_file = File::open("/usr/share/dict/words").unwrap();
    println!("{:?}", "hey");
    let words_reader = BufReader::new(words_file);
    //
    // for line in f.lines() {
    //     println!("{}", line.unwrap());
    // }
}
