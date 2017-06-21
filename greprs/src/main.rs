use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    let query = &args[1];
    let target = &args[2];
    println!("Searching for '{}' in file '{}'", query, target);
    let mut file = File::open(target).expect(format!("File '{}' not found", target).as_str());
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect(format!("error reading file '{}'", target).as_str());
    println!("With contents\n{}", contents);
    // for arg in env::args() {
    //     println!("{:?}", arg);
    // }
}
