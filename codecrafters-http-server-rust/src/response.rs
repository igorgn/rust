use crate::httpparser::{HttpMethod, Request};
use std::{
    collections::HashMap,
    fs::File,
    io::{Read, Write},
    path::{Path, PathBuf},
};

#[derive(Debug)]
pub struct Response {
    status_code: u16,
    status_text: String,
    headers: HashMap<String, String>,
    pub body: Vec<u8>,
    pub binary_body: bool,
}

impl Response {
    pub fn new(status_code: u16, status_text: &str) -> Self {
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "text/plain".to_string());
        Self {
            status_code,
            status_text: status_text.to_string(),
            headers,
            body: Vec::new(),
            binary_body: false,
        }
    }

    pub fn not_found() -> Self {
        Response::new(404, "Not Found").with_body(b"Not Found")
    }

    pub fn with_body(mut self, body: &[u8]) -> Self {
        if self.body.is_empty() {
            self.body = body.to_vec(); //.trim().as_bytes().to_vec();
            self.headers
                .insert("Content-Length".to_string(), body.len().to_string());
        }
        self
    }

    pub fn with_file(mut self, file: &str, file_dir: String) -> Self {
        let path = Path::new(&file_dir).join(file);
        match File::open(path) {
            Ok(mut f) => {
                let mut contents = Vec::new();
                match f.read_to_end(&mut contents) {
                    Ok(_) => {
                        self.headers.insert(
                            "Content-Type".to_string(),
                            "application/octet-stream".to_string(),
                        );
                        self.headers
                            .insert("Content-Length".to_string(), contents.len().to_string());
                        self.body = contents;
                        self
                    }
                    Err(_) => Response::not_found(),
                }
            }
            Err(_) => Response::not_found(),
        }
    }

    pub fn with_header(mut self, key: &str, value: &str) -> Self {
        self.headers.insert(key.to_string(), value.to_string());
        self
    }

    pub fn to_http_string(&self) -> String {
        let mut response = format!("HTTP/1.1 {} {}\r\n", self.status_code, self.status_text);
        for (key, value) in &self.headers {
            response.push_str(&format!("{}: {}\r\n", key, value));
        }
        response.push_str("\r\n");
        // if !self.binary_body {
        //     response.push_str(&String::from_utf8_lossy(&self.body).to_string());
        // }
        response
    }

    fn write_file(&self, file_path: PathBuf, body: Option<String>) -> Response {
        if let Some(body) = body {
            match File::create(file_path) {
                Ok(mut file) => {
                    file.write_all(body.as_bytes()).unwrap();
                    return Response::new(201, "Created\r\n\r\n");
                }
                Err(_) => {}
            }
        }
        Response::default()
    }
}

impl Default for Response {
    fn default() -> Self {
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "text/plain".to_string());
        Self {
            status_code: 200,
            status_text: "OK".to_string(),
            headers,
            body: vec![],
            binary_body: false,
        }
    }
}

pub fn build_response(request: Request, file_dir: String) -> Response {
    let mut parts = request.path.split("/");
    parts.next();
    let path = parts
        .next()
        .map(|p| if p.is_empty() { "/" } else { p })
        .unwrap();
    let arg = parts.next().unwrap_or("").trim();
    println!("Parsed: {}, {}", path, arg);

    let response = Response::default();
    let response = if let Some(encoding_header) = request.headers.get("Accept-Encoding") {
        let encodings: Vec<_> = encoding_header.split(',').collect();
        if encodings.iter().any(|enc| enc.trim() == "gzip") {
            let mut response = response.with_header("Content-Encoding", "gzip");
            use flate2::write::GzEncoder;
            use flate2::Compression;
            let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
            encoder.write_all(arg.as_bytes()).unwrap();
            let e_str = encoder.finish().unwrap();
            response.binary_body = true;
            response.with_body(&e_str)
        } else {
            response
        }
    } else {
        response
    };

    let response = if request
        .headers
        .get("Connection")
        .map(|h| h.eq_ignore_ascii_case("close"))
        == Some(true)
    {
        response.with_header("Connection", "close")
    } else {
        response
    };

    match request.method {
        HttpMethod::GET => match path {
            "/" => response,
            "echo" if !arg.is_empty() => response.with_body(arg.as_bytes()),
            "user-agent" => response.with_body(
                request
                    .headers
                    .get("User-Agent")
                    .unwrap_or(&"No Header")
                    .as_bytes(),
            ),
            "files" => response.with_file(arg, file_dir),
            _ => Response::not_found(),
        },
        HttpMethod::POST => match path {
            "files" => response.write_file(Path::new(&file_dir).join(arg), request.body),
            _ => Response::not_found(),
        },
        HttpMethod::UNKNOWN => Response::not_found(),
    }
}
