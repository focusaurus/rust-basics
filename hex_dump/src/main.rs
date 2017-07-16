extern crate hex_dump;

use std::io;
use std::io::Read;
// use std::io::Write;
use std::io::ErrorKind;

fn main() {
    // let stdout = io::stdout();
    for item in io::stdin().bytes().enumerate() {
        match item {
            (index, Ok(byte)) => {
                print!("{}", hex_dump::format_in_place(index, byte));
            }
            (_, Err(err)) => {
                match err.kind() {
                    ErrorKind::BrokenPipe => print!("hi"),
                    _ => println!("{}", err),
                }
            }
        }
    }
}
