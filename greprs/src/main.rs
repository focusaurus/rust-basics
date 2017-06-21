use std::env;
use std::fs::File;
use std::io::prelude::*;
// use std::path::Path;
// use std::ffi::OsStr;
use std::process;
use std::error::Error;

fn main() {
    let args: Vec<String> = env::args().collect();
    let conf = Conf::new(&args).unwrap_or_else(|err| {
                                                   println!("{}", err);
                                                   process::exit(1);
                                               });
    println!("{:?}", args);
    if let Err(err) = run(conf) {
        println!("{}", err);
        process::exit(2);
    }
}

fn run(conf: Conf) -> Result<(), Box<Error>> {
    println!("Searching for '{}' in file '{}'",
             conf.query,
             conf.input_path);
    let mut file = File::open(conf.input_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    println!("With text:\n{}", contents);
    Ok(())
}

struct Conf {
    query: String,
    input_path: String,
}

// impl Conf {
//     fn new<'a>(args: &'a [String]) -> Result<Conf, &'a str> {
//         let program = Path::new(&args[0])
//             .file_name()
//             .unwrap_or(OsStr::new("greprs"))
//             .to_str()
//             .unwrap();
//         if args.len() < 3 {
//             let message = format!("Usage: {} <query> <input-path>", program).as_str();
//             // return Err(message);
//             // let &'static message  str= format!("Usage: {} <query> <input-path>", program).as_str();
//             // return Err("Usage: greprs <query> <input-path>");
//             return Err(message);
//         }
//         Ok(Conf {
//                query: args[1].clone(),
//                input_path: args[2].clone(),
//            })
//     }
// }
impl Conf {
    fn new(args: &[String]) -> Result<Conf, &'static str> {
        if args.len() < 3 {
            return Err("Usage: greprs <query> <input-path>");
        }
        Ok(Conf {
               query: args[1].clone(),
               input_path: args[2].clone(),
           })
    }
}
