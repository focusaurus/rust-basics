use std::env;
use std::i32;
use std::process::exit;
use std::str::FromStr;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <passenger_count>", args[0]);
        exit(1);
    }
    let count = match i32::from_str(args[1].as_str()) {
        Ok(n) => n,
        Err(_) => 0,
    };
    let vehicle = match count {
        1 => "bike",
        2...5 => "car",
        6...8 => "van",
        9...50 => "bus",
        _ => " fleet of buses",
    };
    let message = match count {
        i32::MIN...-1 => "Soylent is negative people!".to_string(),
        0 => "Nobody home".to_string(),
        1 => format!("{} person needs a {}", count, vehicle),
        _ => format!("{} people need a {}", count, vehicle),
    };

    println!("{}.", message);
}
