use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::fs::File;
use std::thread;
use std::time::Duration;

static LINE: &'static str = "\r\n";
// static HTTP: &'static str = "HTTP/1.1";
static GET: &'static [u8] = b"GET / HTTP/";
static SLEEP: &'static [u8] = b"GET /sleep HTTP/";

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    let mut body = String::new();
    let (status_code, status_text, file_name) = if buffer.starts_with(GET) {
        (200, "OK", "index.html")
    } else if buffer.starts_with(SLEEP) {
        thread::sleep(Duration::from_secs(5));
        (200, "OK", "index.html")
    }else {
        (400, "Not Found", "404.html")
    };
    let mut file = File::open(file_name).unwrap();
    file.read_to_string(&mut body).unwrap();
    let response = format!("HTTP/1.1 {status_code} {status_text}{l}Content-Length: {content_length}{l}{l}{body}",
                           status_code = status_code,
                           status_text = status_text,
                           l = LINE,
                           content_length = body.len(),
                           body = body);
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
