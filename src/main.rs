use http_server_starter_rust::http::*;
use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let raw_request: Vec<String> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let request = HttpRequest::from(raw_request);

    let response = match request.target.as_str() {
        "/" => HttpResponse::ok(None, None),
        path if path.starts_with("/echo/") => HttpResponse::echo(&request),
        _ => HttpResponse::not_found(),
    };

    stream.write_all(&response.as_bytes()).unwrap();
}

// TODO:
// generalize common responses such as common headers
// use &str instead of Strings
// add error handling
// add doc comments:
// https://doc.rust-lang.org/reference/comments.html
// https://doc.rust-lang.org/rustdoc/how-to-write-documentation.html
fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("accepted new connection");
                handle_connection(stream);
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
