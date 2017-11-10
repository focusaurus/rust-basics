use std::env;
use std::error::Error;
use std::fs;
use std::io;
use std::io::ErrorKind;
use std::io::prelude::*;
use std::io::SeekFrom;
use std::path::Path;

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
        let program_name = Path::new(&args[0])
            .file_name()
            .ok_or(to_err("Invalid Filename"))?
            // todo look at to_string_lossy
            // .to_string_lossy()
            .to_str()
            .ok_or(to_err("Filename not valid UTF-8"))?;

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

fn main() {
    line_in().map_err(bail).unwrap();
}
