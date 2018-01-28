extern crate web_service;
use web_service::ThreadPool;

use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::fs::File;
use std::time::Duration;
use std::thread;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    println!("request:{}", String::from_utf8_lossy(&buffer[..]));

    let root = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    if buffer.starts_with(root) {
        make_ok_esponse(stream);
    } else if buffer.starts_with(sleep) {
        make_sleep_esponse(stream);
    } else {
        make_404_esponse(stream);
    }
}

fn make_ok_esponse(stream: TcpStream) {
    let status_ine = "HTTP/1.1 200 OK\r\n\r\n";
    let file = "hello.html";
    make_response(stream, status_ine, file);
}

fn make_sleep_esponse(stream: TcpStream) {
    thread::sleep(Duration::from_secs(5));
    let status_ine = "HTTP/1.1 200 OK\r\n\r\n";
    let file = "hello.html";
    make_response(stream, status_ine, file);
}

fn make_404_esponse(stream: TcpStream) {
    let status_ine = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
    let file = "404.html";
    make_response(stream, status_ine, file);
}

fn make_response(mut stream: TcpStream, status_ine: &str, file: &str) {
    let mut file = File::open(file).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    let response = format!("{}{}", status_ine, content);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
