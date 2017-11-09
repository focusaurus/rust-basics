use std::env;
use std::io;
use std::io::prelude::*;
use std::path::Path;
use std::fs;
use std::io::ErrorKind;
use std::error::Error;
use std::io::SeekFrom;

fn error_exit(message: String, code: i32) {
    write!(&mut io::stderr(), "{}\n", message).unwrap();
    std::process::exit(code);
}

fn bail(err: std::io::Error) {
    let message = match err.kind() {
        ErrorKind::NotFound => "Target file not found",
        _ => err.description(),
    };
    error_exit(String::from(message), 10);
}

fn has_line(target_path: &str, to_add: &str) -> bool {
    let target_file = fs::File::open(target_path).map_err(bail).unwrap();
    io::BufReader::new(&target_file)
        .lines()
        .any(|line| line.unwrap().as_str() == to_add)
}

fn main() {
    println!("main4");
    let args = env::args().collect::<Vec<String>>();
    println!("args {:?}", args);
    if args.len() < 3 {
        let program_name = Path::new(&args[0])
            .file_name()
            .unwrap()
            .to_str()
            .unwrap();

        error_exit(format!("Usage: {} <target_file> <line_to_add>\n", program_name), 1);
    }
    let target_path = &args[1];
    let to_add = &args[2];
    if !has_line(target_path, to_add) {
        println!("need to add");
        let mut target_file = fs::File::open(target_path).map_err(bail).unwrap();
        target_file.seek(SeekFrom::End(0));
        target_file.write(&to_add.clone().into_bytes()).unwrap();
    } else {
        println!("line is already there");
    }
}
