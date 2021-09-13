use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

enum HttpStatus {
    Ok,
    NotFound,
}

fn http_status(status: HttpStatus) -> &'static str {
    match status {
        HttpStatus::Ok => "HTTP/1.1 200 OK",
        HttpStatus::NotFound => "HTTP/1.1 404 NOT FOUND"
    }
}

enum Page {
    Index,
    NotFound,
}
fn page_filename(page: Page) -> &'static str {
    match page {
        Page::Index => "index.html",
        Page::NotFound => "404.html"
    }
}
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        (http_status(HttpStatus::Ok), page_filename(Page::Index))
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        (http_status(HttpStatus::Ok), page_filename(Page::Index))
    } else {
        (http_status(HttpStatus::NotFound), page_filename(Page::NotFound))
    };

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
