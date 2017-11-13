#[macro_use]
extern crate clap;
use clap::{App, Arg};
use std::{fs, io, process};
use std::error::Error;
use std::io::ErrorKind;
use std::io::prelude::*;
use std::io::SeekFrom;

fn has_line(target_file: &std::fs::File, to_add: &str) -> bool {
    io::BufReader::new(target_file)
        .lines()
        .any(|line| line.unwrap().as_str() == to_add)
}

fn line_in() -> Result<(), String> {
    let matches = App::new("line-in")
        .version(crate_version!())
        .about("Ensure a file contains a line")
        .arg(Arg::with_name("target_path")
                 .required(true)
                 .takes_value(false)
                 .help("path to the target file"))
        .arg(Arg::with_name("to_add")
                 .required(true)
                 .takes_value(false)
                 .help("line to add"))
        .get_matches();
    // these .unwrap() calls are fine because both arguments are .required(true)
    let target_path = matches.value_of("target_path").unwrap();
    let to_add = matches.value_of("to_add").unwrap();
    let target_file = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open(&target_path)
        .map_err(|e| format!("Error reading target file: {}", e))?;
    if !has_line(&target_file, &to_add) {
        let mut writer = io::BufWriter::new(target_file);
        writer
            .seek(SeekFrom::End(0))
            .map_err(|e| format!("Error seeking: {}", e))?;
        writeln!(writer, "{}", &to_add).map_err(|i| i.description().to_string())
    } else {
        Ok(())
    }
}

fn main() {
    line_in()
        .map_err(|error| {
                     eprintln!("{}", error);
                     process::exit(1);
                 })
        .unwrap();
}
