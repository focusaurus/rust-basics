extern crate hex_dump;

use std::io;
use std::io::Read;

fn main() {
    // io::stdin()
    //     .bytes()
    //     .map(|result| match result {
    //              Ok(byte) => format_byte(byte),
    //              Err(msg) => String::from("hey"),
    //          });
    for item in io::stdin().bytes().enumerate() {
        match item {
            (index, Ok(byte)) => {
                // interior bytes just need space
                let mut prefix = String::from(" ");
                let offset = hex_dump::format_offset(index);
                if index % 16 == 0 {
                    if index == 0 {
                        // zero we need just offset space
                        prefix = format!("{} ", offset);
                    } else {
                        // other rows we need newline offset space
                        prefix = format!("\n{} ", offset);
                    }
                }
                print!("{}{}", prefix, hex_dump::format_byte(byte));
            }
            (_, Err(msg)) => println!("{}", msg),
        }
        // match item {
        //     (index, Ok(byte)) => {
        //         match result {
        //             Ok(byte) => print!(" {}", format_byte(byte)),
        //             Err(msg) => println!("{}", msg),
        //         }
        //     }
        //     None => println!(""),
        // }
    }
}
