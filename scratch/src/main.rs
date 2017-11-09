use std::fs;
use std::io;
use std::io::prelude::*;
use std::iter;

const WORDS_PATH: &str = "/usr/share/dict/words";

fn is_short(word: &String) -> bool {
    word.len() < 7
}

fn unwrap(result: Result<String, io::Error>) -> String {
    result.unwrap()
}

fn main_works_but_code_dupe() {
    let file = fs::File::open(WORDS_PATH).unwrap();
    let reader = io::BufReader::new(&file);
    let count = reader.lines().map(unwrap).filter(is_short).count();
    println!("{:?}", count);

    let mut reader = io::BufReader::new(&file);
    reader.seek(io::SeekFrom::Start(0));
    let sample_size = (0.05 * count as f32) as usize; // 5% sample

    // This chain of iterator logic is duplicated
    for line in reader.lines().map(unwrap).filter(is_short).take(sample_size) {
        println!("{}", line);
    }
}

fn short_lines<'a, T>
    (reader: &'a T)
     -> iter::Filter<std::iter::Map<std::io::Lines<T>, &FnMut(&str, bool)>, &FnMut(&str, bool)>
    where T: io::BufRead
{
    reader.lines().map(unwrap).filter(is_short)
}

fn main_dry() {
    let file = fs::File::open(WORDS_PATH).unwrap();
    let reader = io::BufReader::new(&file);
    let count = short_lines(reader).count();
    println!("{:?}", count);

    // Would like to do this instead:
    let mut reader = io::BufReader::new(&file);
    reader.seek(io::SeekFrom::Start(0));
    let sample_size = (0.05 * count as f32) as usize; // 5% sample
    for line in short_lines(reader).take(sample_size) {
        println!("{}", line);
    }
}


fn main() {
    main_works_but_code_dupe();
}
