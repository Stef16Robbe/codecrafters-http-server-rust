use http_server_starter_rust::types::*;
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

    let http_request = HttpRequest::from(raw_request);

    let response = match http_request.target.as_str() {
        "/" => HttpResponse::new(StatusCode::Ok, Some("OK".to_string()), None, None),
        _ => HttpResponse::new(
            StatusCode::NotFound,
            Some("Not Found".to_string()),
            None,
            None,
        ),
    };

    stream.write_all(&response.as_bytes()).unwrap();
}

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
