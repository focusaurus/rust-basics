use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::fs::File;

static LINE: &'static str = "\r\n\r\n";

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    let get = b"GET / HTTP/";
    let mut contents = String::new();
    let response;
    if buffer.starts_with(get) {
        let mut file = File::open("index.html").unwrap();
        file.read_to_string(&mut contents).unwrap();
        response = format!("HTTP/1.1 200 OK{}{}", LINE, contents);
    } else {
        let mut file = File::open("404.html").unwrap();
        file.read_to_string(&mut contents).unwrap();
        response = format!("HTTP/1.1 404 Not Found{}{}", LINE, contents);
    }
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    for stream in listener.incoming() {
        println!("Client connected");
        handle_connection(stream.unwrap());
    }
}
