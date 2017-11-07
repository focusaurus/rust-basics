// todo generate a pair of random integers
// todo try shuffled_iter
extern crate rand;
extern crate shuffled_iter;
mod scratch;
use shuffled_iter::ShuffledIterGen;

// use random::Source;
use std::fs;
use std::io;
use std::io::prelude::*;

fn main() {
    // scratch::scratch();
    // let mut source = random::default().seed([48654684, 7686898]);
    // let mut index = source.read::<usize>();
    let words_file = fs::File::open("/usr/share/dict/words").unwrap();
    let words_reader = io::BufReader::new(words_file);
    let how_many = 4;
    let short_word_count= words_reader
        .lines()
        .map(|result| result.unwrap())
        .filter(|line| line.len() < 7)
        .count();
    println!("{:?}", short_word_count);
    let mut chaos = rand::thread_rng();
    // let mut pair: [Option<u32>; 2] = [None, None];
    let mut first: Option<u32> = None;
    for index in chaos.iter_shuffled(0u32..short_word_count as u32).take(how_many*2) {
        if first.is_none() {
            first = Some(index);
        } else {
            println!("indices: {}, {}", first.unwrap(), index);
            first = None;
        }
    }
        //
    // ✓ get length of filtered word iter
    // get a shuffle rng with range 0 to wordLength
    // take N out of that to use as indexes where N <= shortWords.len()
    //
    // let word = words_reader
    //     .lines()
    //     .map(|result| result.unwrap())
    //     .filter(|line| line.len() < 8)
    //     .count(index).unwrap();
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
