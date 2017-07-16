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
                match result {
                    Ok(_) => (),
                    Err(err) => {
                        let code = match err.kind() {
                            // This is OK, happens when piped to head
                            ErrorKind::BrokenPipe => 0,
                            _ => {
                                writeln!(stderr, "{}", err).unwrap();
                                1
                            }
                        };
                        process::exit(code);
                    }
                }
            }
            (_, Err(err)) => writeln!(stderr, "{}", err).unwrap(),
        }
    }
}
