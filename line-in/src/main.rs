use std::env;
use std::io;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    println!("main3");
    let args = env::args().collect::<Vec<String>>();
    println!("args {:?}", args);
    if args.len() < 3 {
        let program_name = Path::new(&args[0])
            .file_name()
            .unwrap()
            .to_str()
            .unwrap();
        write!(&mut io::stderr(),
               "Usage: {} <target_file> <line_to_add>\n",
               program_name)
                .unwrap();
        std::process::exit(1);
    }
    println!("Hello, world! {:?}", 45);
}
