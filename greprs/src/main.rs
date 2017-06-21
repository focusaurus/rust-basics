extern crate greprs;
use std::env;
use std::process;
use greprs::Conf;

fn main() {
    let args: Vec<String> = env::args().collect();
    let conf = Conf::new(&args).unwrap_or_else(|err| {
                                                   println!("{}", err);
                                                   process::exit(1);
                                               });
    println!("{:?}", args);
    if let Err(err) = greprs::run(conf) {
        println!("{}", err);
        process::exit(2);
    }
}
