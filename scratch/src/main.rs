#[macro_use]
extern crate clap;
use clap::{Arg, App};

fn tirefox() -> Result<(), clap::Error> {
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
    match tirefox() {
        Err(error) => {
            eprintln!("{}", error.message);
            std::process::exit(1);
        }
        _ => {}
    }
}
