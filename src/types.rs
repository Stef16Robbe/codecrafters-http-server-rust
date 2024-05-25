use std::collections::HashMap;

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
        match &self.reason {
            Some(reason) => write!(
                f,
                "{} {} {}\r\n\r\n",
                self.version, self.status_code as u8, reason
            ),
            None => write!(f, "{} {}\r\n\r\n", self.version, self.status_code as u8),
        }
    }
}

pub type Reason = Option<String>;
pub type Headers = HashMap<String, String>;
pub type Body = String;

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
