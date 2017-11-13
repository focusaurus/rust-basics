#[macro_use]
extern crate clap;

use clap::{Arg, App};
use std::{convert, io};
enum AnyError {
    Io(io::Error),
    Clap(clap::Error),
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
        AnyError::Io(io::Error::new(io::ErrorKind::Other, message))
    }
}


// fn scratch<E> () -> Result<(), E> where E: fmt::Display + convert::From<clap::Error>   {
fn scratch<E> () -> Result<(), AnyError> {
    let matches = App::new("scratch")
        .arg(Arg::with_name("count")
                 .short("c")
                 .long("count")
                 .takes_value(true))
        .get_matches();
    let count = value_t!(matches, "count", usize)?;
    println!("count {}", count);
    Ok(())
}

fn main() {
    match scratch::<clap::Error>() {
        Ok(unit) => unit,
        // Err(ref error) => error.exit(),
        Err<io::Error>(ioe) => {
            eprintln!("{}", ioe.description())
        }
    }
}
