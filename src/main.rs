use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::fs::File;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    println!("request:{}", String::from_utf8_lossy(&buffer[..]));

    if verify(Box::new(buffer)) {
        make_response(stream);
    }
}

fn verify(buffer: Box<[u8]>) -> bool {
    let condition = b"GET / HTTP/1.1\r\n";
    if buffer.starts_with(condition) {
        true
    } else {
        false
    }
}

fn make_response(mut stream: TcpStream) {
    let mut file = File::open("hello.html").unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", content);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}