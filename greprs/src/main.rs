extern crate greprs;
use std::env;
use std::process;
use greprs::Conf;
use std::io::prelude::*;

fn main() {
    let mut stderr = std::io::stderr();
    let conf = Conf::new(env::args()).unwrap_or_else(|err| {
                                                   writeln!(&mut stderr, "{}", err).expect("cannot write to stderr");
                                                   process::exit(1);
                                               });
    if let Err(err) = greprs::run(conf) {
        writeln!(&mut stderr, "{}", err).expect("cannot write to stderr");
        process::exit(2);
    }
}
