extern crate hex_dump;

use std::io;
use std::io::Read;
use std::io::Write;
use std::io::ErrorKind;

fn main() {
    use std::process;
    let mut stdout = io::stdout();
    let mut stderr = io::stderr();
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
                        writeln!(stderr, "{}", err).unwrap();
                    }
                    process::exit(code);
                }
            }
            (_, Err(err)) => writeln!(stderr, "{}", err).unwrap(),
        }
    }
}
