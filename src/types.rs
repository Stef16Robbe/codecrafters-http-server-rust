use std::collections::HashMap;
use std::fmt::Write;

/// An HTTP response is made up of three parts, each separated by a CRLF (\r\n):
///
/// 1. Status line.
/// 2. Zero or more headers, each ending with a CRLF.
/// 3. Optional response body.
///
/// For example:
/// HTTP/1.1 200 OK\r\n\r\n
#[derive(Debug)]
pub struct HttpResponse {
    pub version: HttpVersion,
    pub status_code: StatusCode,
    pub reason: Reason,
    pub headers: Headers,
    pub body: Body,
}

impl HttpResponse {
    pub fn new(status_code: StatusCode, reason: Reason, headers: Headers, body: Body) -> Self {
        HttpResponse {
            version: HttpVersion::Http11,
            status_code,
            reason,
            headers,
            body,
        }
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        self.to_string().into_bytes()
    }
}

impl std::fmt::Display for HttpResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let status_line = match &self.reason {
            Some(reason) => format!(
                "{} {} {}\r\n",
                self.version, self.status_code as u16, reason
            ),
            None => format!("{} {}\r\n", self.version, self.status_code as u16),
        };

        let headers = match &self.headers {
            Some(headers) => headers
                .iter()
                .fold(String::new(), |mut output, (key, value)| {
                    let _ = write!(output, "{}: {}\r\n", key, value);
                    output
                }),
            None => "".into(),
        };

        // end headers section
        let headers = format!("{}\r\n", headers);

        let body = match &self.body {
            Some(body) => body.to_string(),
            None => "".into(),
        };

        write!(f, "{}{}{}", status_line, headers, body)
    }
}

#[derive(Debug)]
pub struct HttpRequest {
    pub method: HttpMethod,
    pub target: String,
    pub version: HttpVersion,
    pub headers: Headers,
    pub body: Body,
}

// ["GET /index.html HTTP/1.1", "Host: localhost:4221", "User-Agent: curl/8.6.0", "Accept: */*"]
impl From<Vec<String>> for HttpRequest {
    // TODO:
    // add err handling
    // account for body
    fn from(data: Vec<String>) -> Self {
        let mut request_line = data[0].split(' ');
        let method = HttpMethod::from(request_line.next().unwrap());
        let target = String::from(request_line.next().unwrap());
        let version = HttpVersion::from(request_line.next().unwrap());

        let headers: HashMap<_, _> = data
            .iter()
            .skip(1)
            .filter_map(|s| {
                s.split_once(' ')
                    .map(|(k, v)| (k.to_string(), v.to_string()))
            })
            .collect();

        HttpRequest {
            method,
            target,
            version,
            headers: Some(headers),
            body: None,
        }
    }
}

#[derive(Debug)]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Patch,
    Delete,
    Connect,
    Head,
    Options,
    Trace,
    NotImp,
}

impl From<&str> for HttpMethod {
    fn from(method: &str) -> Self {
        match method {
            "GET" => HttpMethod::Get,
            "POST" => HttpMethod::Post,
            "PUT" => HttpMethod::Put,
            "PATCH" => HttpMethod::Patch,
            "DELETE" => HttpMethod::Delete,
            "CONNECT" => HttpMethod::Connect,
            "HEAD" => HttpMethod::Head,
            "OPTIONS" => HttpMethod::Options,
            "TRACE" => HttpMethod::Trace,
            _ => HttpMethod::NotImp,
        }
    }
}

pub type Reason = Option<String>;
pub type Headers = Option<HashMap<String, String>>;
pub type Body = Option<String>;

#[derive(Debug)]
pub enum HttpVersion {
    Http10,
    Http11,
    Http20,
    Unknown,
}

impl From<&str> for HttpVersion {
    fn from(method: &str) -> Self {
        match method {
            "HTTP/1.0" => HttpVersion::Http10,
            "HTTP/1.1" => HttpVersion::Http11,
            "HTTP/2.0" => HttpVersion::Http20,
            _ => HttpVersion::Unknown,
        }
    }
}

impl std::fmt::Display for HttpVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            HttpVersion::Http10 => write!(f, "HTTP/1.0"),
            HttpVersion::Http11 => write!(f, "HTTP/1.1"),
            HttpVersion::Http20 => write!(f, "HTTP/2.0"),
            HttpVersion::Unknown => write!(f, ""),
        }
    }
}

#[repr(u16)]
#[derive(Debug, Clone, Copy)]
pub enum StatusCode {
    Ok = 200,
    BadRequest = 400,
    NotFound = 404,
    InternalServerError = 500,
}

impl StatusCode {
    pub fn from_u16(code: u16) -> Option<Self> {
        match code {
            200 => Some(StatusCode::Ok),
            400 => Some(StatusCode::BadRequest),
            404 => Some(StatusCode::NotFound),
            500 => Some(StatusCode::InternalServerError),
            _ => None,
        }
    }
}
