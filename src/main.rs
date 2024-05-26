use http_server_starter_rust::types::*;
use std::{
    collections::HashMap,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn respond_ok() -> HttpResponse {
    HttpResponse::new(StatusCode::Ok, Some("OK".to_string()), None, None)
}

// TODO:
// generalize common responses
// such as common headers
// fix header Display
// use &str instead of Strings
// add error handling
fn respond_echo(request: &HttpRequest) -> HttpResponse {
    let res_body = request.target.split('/').last().unwrap();
    let headers = HashMap::from([
        ("Content-Type".to_string(), "text/plain".to_string()),
        ("Content-Length".to_string(), res_body.len().to_string()),
    ]);
    HttpResponse::new(
        StatusCode::Ok,
        Some("OK".to_string()),
        Some(headers),
        Some(res_body.to_string()),
    )
}

fn respond_notfound() -> HttpResponse {
    HttpResponse::new(
        StatusCode::NotFound,
        Some("Not Found".to_string()),
        None,
        None,
    )
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let raw_request: Vec<String> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let request = HttpRequest::from(raw_request);

    let response = match request.target.as_str() {
        "/" => respond_ok(),
        path if path.starts_with("/echo/") => respond_echo(&request),
        _ => respond_notfound(),
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
