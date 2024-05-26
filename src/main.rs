use http_server_starter_rust::http::*;
use std::{
    env,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn handle_files(request: &HttpRequest, working_dir: &str) -> HttpResponse {
    let response = match request.method {
        HttpMethod::Get => HttpResponse::get_file(request, working_dir),
        HttpMethod::Post => HttpResponse::post_file(request, working_dir),
        _ => Ok(HttpResponse::not_found()),
    };

    match response {
        Ok(res) => res,
        Err(err) => match err {
            HttpRequestError::BadRequest(e) => {
                println!("error: {:?}", e);
                HttpResponse::bad_request(None, None)
            }
            HttpRequestError::InternalServerError(e) => {
                println!("error: {:?}", e);
                HttpResponse::internal_server_error()
            }
            HttpRequestError::NotFound(e) => {
                println!("error: {:?}", e);
                HttpResponse::not_found()
            }
        },
    }
}

fn handle_connection(mut stream: TcpStream, working_dir: &str) {
    let buf_reader = BufReader::new(&mut stream);
    let raw_request: Vec<String> = buf_reader.lines().map(|result| result.unwrap()).collect();

    let request = HttpRequest::try_from(raw_request);

    let response = match request {
        Ok(req) => match req.target.as_str() {
            "/" => HttpResponse::ok(None, None),
            "/user-agent" => match HttpResponse::user_agent(&req) {
                Ok(res) => res,
                Err(e) => {
                    println!("error: {:?}", e);
                    HttpResponse::bad_request(None, None)
                }
            },
            path if path.starts_with("/echo/") => match HttpResponse::echo(&req) {
                Ok(res) => res,
                Err(e) => {
                    println!("error: {:?}", e);
                    HttpResponse::bad_request(None, None)
                }
            },
            path if path.starts_with("/files/") => handle_files(&req, working_dir),
            _ => HttpResponse::not_found(),
        },
        Err(e) => {
            println!("error: {:?}", e);
            HttpResponse::bad_request(None, None)
        }
    };

    stream.write_all(&response.as_bytes()).unwrap();
}

// TODO:
// add proper logging
// fix error handling, fix nested `match` statements
// add doc comments:
// https://doc.rust-lang.org/reference/comments.html
// https://doc.rust-lang.org/rustdoc/how-to-write-documentation.html
// use &str instead of Strings
fn main() {
    let args: Vec<String> = env::args().collect();
    println!("executing with args: {args:?}");
    let working_dir = if args.len() > 1 { args[2].as_str() } else { "" };

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("accepted new connection");
                handle_connection(stream, working_dir);
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
