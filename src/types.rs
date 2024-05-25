use std::collections::HashMap;

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
    pub fn new(
        version: HttpVersion,
        status_code: StatusCode,
        reason: Reason,
        headers: Headers,
        body: Body,
    ) -> Self {
        HttpResponse {
            version,
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
            // TODO:
            // fix display for headers
            Some(headers) => format!("{:?}\r\n", headers),
            None => "\r\n".into(),
        };

        let body = match &self.body {
            Some(body) => format!("{}", body),
            None => "".into(),
        };

        write!(f, "{}{}{}", status_line, headers, body)
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
}

impl std::fmt::Display for HttpVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            HttpVersion::Http10 => write!(f, "HTTP/1.0"),
            HttpVersion::Http11 => write!(f, "HTTP/1.1"),
            HttpVersion::Http20 => write!(f, "HTTP/2.0"),
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
