use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    let get = b"GET ";
    let post = b"POST ";
    let delete = b"DELETE ";

    let (method, url) = if buffer.starts_with(get) {
        ("GET", parse_url(&buffer, 4))
    } else if buffer.starts_with(post) {
        ("POST", parse_url(&buffer, 5))
    } else if buffer.starts_with(delete) {
        ("DELETE", parse_url(&buffer, 7))
    } else {
        ("", "")
    };
    
    let url = url.split_whitespace().next().unwrap_or("/");
    println!("HTTP Method: {:?}, URL: {:?}", method, url);

    let (status_line, filename, content_type) = match (method, url) {
        ("GET", "/") => ("HTTP/1.1 200 OK", "index.html", "text/html; charset=utf-8"),
        ("GET", "/styles.css") => ("HTTP/1.1 200 OK", "styles.css", "text/css; charset=utf-8"),
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html", "text/html; charset=utf-8"),
    };

    let mut contents = std::fs::read_to_string(filename).unwrap();
    contents = contents.replace("{}", "{ \"success\": \"SSR\" }");
    contents = contents.replace("{{ success }}", "SSR");
    
    let mut response = format!(
        "{}\r\nContent-Length: {}\r\nContent-Type: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        content_type,
        contents
    );

    if content_type == "text/css; charset=utf-8" {
        response = format!(
            "{}\r\nContent-Length: {}\r\nContent-Type: {}\r\nCache-Control: {}\r\n\r\n{}",
            status_line,
            contents.len(),
            content_type,
            "max-age=2592000",
            contents
        );
    }
    
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn parse_url(buffer: &[u8], start_index: usize) -> &str {
    let end_index = buffer.iter().position(|&x| x == b'\r' || x == b'\n').unwrap_or_else(|| buffer.len());
    std::str::from_utf8(&buffer[start_index..end_index]).unwrap().trim()
}