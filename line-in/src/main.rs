#[macro_use]
extern crate clap;
use clap::{App, Arg};
use std::env;
use std::error::Error;
// use std::ffi::OsStr;
use std::{convert, fs, io, process};
use std::io::ErrorKind;
use std::io::prelude::*;
use std::io::SeekFrom;
use std::path::Path;

enum AnyError {
    Io(io::Error),
    Clap(clap::Error),
}

fn error_exit(message: String, code: i32) {
    eprintln!("{}", message);
    std::process::exit(code);
}

fn bail(err: std::io::Error) {
    let message = match err.kind() {
        ErrorKind::NotFound => "Target file not found",
        _ => err.description(),
    };
    error_exit(String::from(message), 10);
}

fn has_line(target_file: &std::fs::File, to_add: &str) -> bool {
    io::BufReader::new(target_file)
        .lines()
        .any(|line| line.unwrap().as_str() == to_add)
}

fn to_err(message: &str) -> io::Error {
    io::Error::new(ErrorKind::InvalidData, message)
}

fn line_in() -> io::Result<i32> {
    let args = env::args().collect::<Vec<String>>();
    if args.len() < 3 {
        let program_name = args.first()
            .map(Path::new)
            .and_then(Path::file_name)
            .map(|o| o.to_string_lossy().into_owned())
            .unwrap_or_else(|| "UNKNOWN".to_string());
        error_exit(format!("Usage: {} <target_file> <line_to_add>\n", program_name),
                   1);
    }
    let target_path = &args[1];
    let to_add = &args[2];

    let target_file = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open(&target_path)?;
    if !has_line(&target_file, &to_add) {
        let mut writer = io::BufWriter::new(target_file);
        writer.seek(SeekFrom::End(0))?;
        writeln!(writer, "{}", &to_add);
        writer.flush()? // possibly unnecessary???
    }
    Ok(0)
}

impl convert::From<clap::Error> for AnyError {
    fn from(error: clap::Error) -> Self {
        AnyError::Clap(error)
    }
}


impl convert::From<io::Error> for AnyError {
    fn from(error: io::Error) -> Self {
        AnyError::Io(error)
    }
}
impl convert::From<&'static str> for AnyError {
    fn from(message: &str) -> Self {
        AnyError::Io(io::Error::new(ErrorKind::Other, message))
    }
}

fn line_in2() -> Result<(), String> {
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
        .open(&target_path).map_err(|e|"Error reading target file".to_string())?;
    if !has_line(&target_file, &to_add) {
        let mut writer = io::BufWriter::new(target_file);
        writer.seek(SeekFrom::End(0)).map_err(|e|"IO error".to_string())?;
        writeln!(writer, "{}", &to_add);
        // writer.flush()? // possibly unnecessary???
    }
    Ok(())
}

fn main() {
    match line_in2() {
        Ok(file) => {
            println!("line-in OK");
        }
        Err(error) => {
            eprintln!("exiting {}", error);
            process::exit(1);
        }
    }}
