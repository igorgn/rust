use std::collections::HashMap;

use crate::errors::RequestParseError;

#[derive(Debug)]
pub struct Request<'h> {
    pub method: HttpMethod,
    pub path: &'h str,
    pub version: u8,
    pub headers: HashMap<&'h str, &'h str>,
    pub body: Option<String>,
}

impl<'h> Request<'h> {
    pub fn parse_headers(
        buf: &'h str,
        headers: &'h [String],
    ) -> Result<Request<'h>, RequestParseError> {
        let mut parts = buf.trim_end_matches("\r\n").split_whitespace();

        let method = HttpMethod::from(parts.next().ok_or(RequestParseError::MissingMethod)?);
        let path = parts.next().ok_or(RequestParseError::MissingPath)?;
        let version = parts.next().ok_or(RequestParseError::MissingVersion)?;
        let version = match version {
            "HTTP/1.0" | "HTTP/1.1" => 1,
            "HTTP/2.0" => 2,
            _ => return Err(RequestParseError::UnsupportedVersion),
        };

        let mut headers_hm = HashMap::new();
        headers.iter().for_each(|s| {
            if let Some(pos) = s.find(":") {
                headers_hm.insert(&s[..pos], s[pos + 1..].trim());
            }
        });

        Ok(Request {
            method,
            path,
            version,
            headers: headers_hm,
            body: None,
        })
    }

    pub fn body(&mut self, body: Vec<u8>) {
        self.body = Some(String::from_utf8_lossy(&body).to_string());
        // self
    }
}

#[derive(Debug, PartialEq)]
pub enum HttpMethod {
    GET,
    POST,
    UNKNOWN,
}

impl From<&str> for HttpMethod {
    fn from(value: &str) -> Self {
        match value {
            "GET" => HttpMethod::GET,
            "POST" => HttpMethod::POST,
            _ => HttpMethod::UNKNOWN,
        }
    }
}
