use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::ffi::OsStr;

fn main() {
    let args: Vec<String> = env::args().collect();
    let conf = Conf::new(&args);
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
    fn new(args: &[String]) -> Conf {
        // let program = match Path::new(&args[0]).file_name().or(Some(OsStr::new("greprs")) {
        //         None => OsStr::new("greprs"),
        //         Some(osstr) => osstr,
        //     }
        //     .to_str()
        //     .unwrap_or("greprs");
        let program = Path::new(&args[0])
            .file_name()
            .unwrap_or(OsStr::new("greprs"))
            .to_str()
            .unwrap();
        // if let Some(osstr) = Path::new(&args[0]).file_name() {
        //     println!("osstr hey {:?}", osstr);
        // } else {
        //     println!("none");
        // }

        // .or(OsStr::new("greprs").to_str());
        if args.len() < 3 {
            panic!("Usage: {} <query> <input-path>", program);
        }
        Conf {
            query: args[1].clone(),
            input_path: args[2].clone(),
        }
    }
}
