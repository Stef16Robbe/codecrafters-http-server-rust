use anyhow::Context;
use std::collections::HashMap;
use std::fmt::Write;
use thiserror::Error;

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

    pub fn ok(headers: Headers, body: Body) -> Self {
        HttpResponse::new(StatusCode::Ok, Some("OK".to_string()), headers, body)
    }

    pub fn bad_request(headers: Headers, body: Body) -> Self {
        HttpResponse::new(
            StatusCode::BadRequest,
            Some("Bad Request".to_string()),
            headers,
            body,
        )
    }

    pub fn not_found() -> HttpResponse {
        HttpResponse::new(
            StatusCode::NotFound,
            Some("Not Found".to_string()),
            None,
            None,
        )
    }

    pub fn echo(request: &HttpRequest) -> anyhow::Result<HttpResponse> {
        let res_body = request
            .target
            .split('/')
            .last()
            .context("could not get last element of /echo/ endpoint")
            .unwrap();

        let headers = HashMap::from([
            ("Content-Type".to_string(), "text/plain".to_string()),
            ("Content-Length".to_string(), res_body.len().to_string()),
        ]);

        Ok(HttpResponse::new(
            StatusCode::Ok,
            Some("OK".to_string()),
            Some(headers),
            Some(res_body.to_string()),
        ))
    }

    pub fn user_agent(request: &HttpRequest) -> Result<HttpResponse, HttpRequestError> {
        let agent_header = match &request.headers {
            Some(headers) => match headers.get("User-Agent") {
                Some(hdr) => hdr,
                None => {
                    return Err(HttpRequestError::BadRequest(
                        "missing user-agent header".to_string(),
                    ))
                }
            },
            None => return Err(HttpRequestError::BadRequest("missing headers".to_string())),
        };

        let headers = HashMap::from([
            ("Content-Type".to_string(), "text/plain".to_string()),
            ("Content-Length".to_string(), agent_header.len().to_string()),
        ]);

        Ok(HttpResponse::new(
            StatusCode::Ok,
            Some("OK".to_string()),
            Some(headers),
            Some(agent_header.to_string()),
        ))
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

impl TryFrom<Vec<String>> for HttpRequest {
    type Error = HttpRequestError;

    // TODO:
    // account for body
    fn try_from(data: Vec<String>) -> Result<Self, HttpRequestError> {
        let mut request_line = data[0].split(' ');
        if request_line.clone().count() != 3 {
            return Err(HttpRequestError::BadRequest(
                "request line is malformed".to_string(),
            ));
        }

        let method = HttpMethod::from(request_line.next().unwrap());
        let target = String::from(request_line.next().unwrap());
        let version = HttpVersion::from(request_line.next().unwrap());

        let headers: HashMap<_, _> = data
            .iter()
            .skip(1)
            .filter_map(|s| {
                s.split_once(": ")
                    .map(|(k, v)| (k.to_string(), v.to_string()))
            })
            .collect();

        Ok(HttpRequest {
            method,
            target,
            version,
            headers: Some(headers),
            body: None,
        })
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

#[derive(Error, Debug)]
pub enum HttpRequestError {
    #[error("bad request")]
    BadRequest(String),
    #[error("internal server error")]
    InternalServerError(String),
}
