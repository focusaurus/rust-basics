// use std::ffi::OsStr;
// use std::path::Path;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::env;

pub fn run(conf: Conf) -> Result<(), Box<Error>> {
    let mut file = File::open(conf.input_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let matches = if conf.case_sensitive {
        search(&conf.query, &contents)
    } else {
        search_case_insensitive(&conf.query, &contents)
    };
    for line in matches {
        println!("{}", line);
    }
    Ok(())
}

pub struct Conf {
    pub query: String,
    pub input_path: String,
    pub case_sensitive: bool,
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
    pub fn new(mut args: std::env::Args) -> Result<Conf, &'static str> {
        args.next(); // discard the program name
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };
        let input_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };
        Ok(Conf {
               query: query,
               input_path: input_path,
               case_sensitive: env::var("CASE_INSENSITIVE").is_err(),
           })
    }
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut matches = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            matches.push(line);
        }
    }
    matches
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut matches = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query.to_lowercase()) {
            matches.push(line)
        }
    }
    matches
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(vec!["Rust:", "Trust me."],
                   search_case_insensitive(query, contents));
    }
}
