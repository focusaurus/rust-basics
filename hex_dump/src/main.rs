extern crate hex_dump;

use std::io;
use std::io::{ErrorKind,Read,Write};

fn main() {
    use std::process;
    let mut stdout = io::stdout();
    for item in io::stdin().bytes().enumerate() {
        match item {
            (index, Ok(byte)) => {
                let result = write!(stdout, "{}", hex_dump::format_in_place(index, byte));
                if let Err(err) = result {
                    let mut code = 1;
                    if let ErrorKind::BrokenPipe = err.kind() {
                        // This is OK, happens when piped to head
                        code = 0;
                    } else {
                        eprintln!("{}", err);
                    }
                    process::exit(code);
                }
            }
            (_, Err(err)) => eprintln!("{}", err),
        }
    }
}
