extern crate rand;
extern crate shuffled_iter;
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

fn bail_unwrap(result: Result<String, io::Error>) -> String {
    result.map_err(bail).unwrap()
}

fn sorted_indices(max: usize) -> Vec<usize> {
    let mut chaos = rand::thread_rng();
    let mut indices: Vec<usize> = chaos
        .iter_shuffled(0usize..max as usize)
        .take(HOW_MANY * 2)
        .collect();
    indices.sort();
    indices
}

// fn index_pairs<'a>(indices: &'a Vec<usize>, iter: &mut Iterator<Item=&'a str>) -> &'a Vec<&'a str> {
//         let mut last = 0;
//         let results = &indices
//                            .iter()
//                            .map(|&index| {
//                                     let word = iter.nth(index - last - 1).unwrap();
//                                     last = index;
//                                     word
//                                 })
//                            .collect::<Vec<&'a str>>();
// results
// }
//
// fn iter_short_lines(reader: &io::BufReader<fs::File>) -> &std::iter::Filter<> {
//             reader.lines()
//             .map(bail_unwrap)
//             .filter(is_short)
// }

fn main() {
    // count total words (one per line) in the file
    let words_file = fs::File::open("/usr/share/dict/words")
        .map_err(bail)
        .unwrap();
    let words_reader = io::BufReader::new(&words_file);
    let short_word_count = words_reader
        .lines()
        .map(bail_unwrap)
        .filter(is_short)
        .count();

    // Prepare the iterator of the words themselves
    let mut reader = io::BufReader::new(&words_file);
    reader.seek(io::SeekFrom::Start(0));
    let mut short_word_iter = reader
        .lines()
        .map(bail_unwrap)
        .filter(is_short);

    let mut indices = sorted_indices(short_word_count);
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
