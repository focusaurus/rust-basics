use std::io::prelude::*;
use std::net::{TcpListener,TcpStream};
use std::fs::File;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    for stream in listener.incoming() {
        println!("Client connected");
        handle_connection(stream.unwrap());
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    let mut file = File::open("index.html").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let response =format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
