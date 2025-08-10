mod errors;
mod httpparser;
mod response;
mod threadpool;

#[allow(unused_imports)]
use std::net::TcpListener;
use std::{
    io::{BufRead, BufReader, Read, Write},
    net::TcpStream,
};

use crate::{response::build_response, threadpool::ThreadPool};
use clap::Parser;
use errors::WebServerError;
use httpparser::Request;

#[derive(Parser, Debug)]
struct Cli {
    #[clap(long)]
    directory: Option<String>,
}

// TODO: Refactor. Improve error handling! Modules
fn main() -> Result<(), WebServerError> {
    let cli = Cli::parse();
    let file_dir = cli.directory.unwrap_or("".to_string());
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    let pool = ThreadPool::new(5);

    for stream in listener.incoming() {
        let file_dir = file_dir.clone();
        match stream {
            Ok(stream) => {
                pool.execute(|| {
                    handle_connection(stream, file_dir)
                        .map_err(|_| WebServerError::StreamError)
                        .unwrap()
                });
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
    Ok(())
}

fn handle_connection(mut stream: TcpStream, file_dir: String) -> std::io::Result<()> {
    loop {
        let mut stream_reader = BufReader::new(&stream);
        let mut request_line = String::new();
        stream_reader.read_line(&mut request_line)?;

        let mut headers = Vec::new();
        loop {
            let mut header_line = String::new();
            stream_reader.read_line(&mut header_line)?;

            if header_line.trim().is_empty() {
                break;
            }

            headers.push(header_line);
        }

        let mut request = match Request::parse_headers(&request_line, &headers) {
            Ok(req) => req,
            Err(e) => {
                eprintln!("{}", e);
                stream.write(b"HTTP/1.1 400 Bad Request\r\n\r\n")?;
                return Ok(());
            }
        };

        if let Some(&cl) = request.headers.get("Content-Length") {
            let content_length = cl.trim().parse::<usize>().unwrap();
            let mut body = vec![0; content_length];
            stream_reader.read_exact(&mut body)?;

            request.body(body);
        }

        let close_connection = request
            .headers
            .get("Connection")
            .map(|h| h.eq_ignore_ascii_case("close"))
            == Some(true);
        let response = build_response(request, file_dir.clone());

        println!("Sending response {:#?} to client...", response);
        stream.write_all(response.to_http_string().as_bytes())?;
        // if response.binary_body {
        stream.write_all(&response.body)?;

        if close_connection {
            break Ok(());
        }
    }
}
