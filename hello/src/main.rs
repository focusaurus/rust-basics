use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::fs::File;

static LINE: &'static str = "\r\n\r\n";
static GET: &'static [u8] = b"GET / HTTP/";

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    let mut contents = String::new();
    let (status, file_name) = if buffer.starts_with(GET) {
        ("HTTP/1.1 200 OK", "index.html")
    } else {
        ("HTTP/1.1 400 Not Found", "404.html")
    };
    let mut file = File::open(file_name).unwrap();
    file.read_to_string(&mut contents).unwrap();
    let response = format!("{}{}{}", status, LINE, contents);
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
