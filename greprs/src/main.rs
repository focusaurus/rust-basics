use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::ffi::OsStr;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let conf = Conf::new(&args).unwrap_or_else(|err| {
                                                   println!("{}", err);
                                                   process::exit(1);
                                               });
    println!("{:?}", args);
    println!("Searching for '{}' in file '{}'",
             conf.query,
             conf.input_path);
    let mut file = File::open(&conf.input_path).expect(format!("File '{}' not found",
                                                               conf.input_path)
                                                               .as_str());
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect(format!("error reading file '{}'", conf.input_path).as_str());
    println!("With contents\n{}", contents);
}

struct Conf {
    query: String,
    input_path: String,
}

impl Conf {
    fn new(args: &[String]) -> Result<Conf, &'static str> {
        let program = Path::new(&args[0])
            .file_name()
            .unwrap_or(OsStr::new("greprs"))
            .to_str()
            .unwrap();
        if args.len() < 3 {
            // let &'static message  str= format!("Usage: {} <query> <input-path>", program).as_str();
            // return Err(message);
            // let &'static message  str= format!("Usage: {} <query> <input-path>", program).as_str();
            return Err("Usage: greprs <query> <input-path>");
        }
        Ok(Conf {
               query: args[1].clone(),
               input_path: args[2].clone(),
           })
    }
}
