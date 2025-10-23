use std::{fs::File, io::Read, path::PathBuf};

use anyhow::{Result};

mod lexer;
mod parser;
pub mod errors;

pub struct Jq {
    json_raw: String,
}

#[derive(Debug)]
struct JsonEntry {
    key: String,
    value: JsonValue,
}

#[derive(Debug)]
enum JsonValue {
    String(String),
    Number(f64),
    Boolean(bool),
    Null,
    Object(Vec<JsonEntry>),
    Array(Vec<JsonValue>),
}

impl Jq {
    pub fn new(file_path: PathBuf) -> Self {
        let mut buf = String::new();
        let mut file = File::open(&file_path).unwrap();
        file.read_to_string(&mut buf).unwrap();

        Self {
            json_raw: buf.trim().to_string(),
        }
    }

    pub fn parse(&self) -> Result<()> {
        let mut lex = lexer::Lexer::new(&self.json_raw);
        while !lex.is_at_end() {
            if let Ok(token) = lex.next_token() {
                println!("{:?}", token)
            }
        }
        Ok(())
    }
}
