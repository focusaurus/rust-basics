use std::fs;
use std::io;
use std::io::prelude::*;

fn main() {
    let file = fs::File::open("/usr/share/dict/words").unwrap();
    let reader = io::BufReader::new(&file);
    let count = reader.lines().count();
    println!("{:?}", count);

    // let file = fs::File::open("/usr/share/dict/words").unwrap();
    let mut reader = io::BufReader::new(&file);
    reader.seek(io::SeekFrom::Start(0));
    for line in reader
            .lines()
            .map(|line| line.unwrap())
            .filter(|line| line.len() > 7) {
        println!("hey {:?}", line);

    }
}
