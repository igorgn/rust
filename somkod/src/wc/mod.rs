// pub mod commands;

use bytes::BytesMut;
use std::{
    fs::File,
    io::{self, Read},
    path::PathBuf,
};

pub struct WC {
    buf: BytesMut,
    file_path: Option<PathBuf>,
    bytes: bool,
    lines: bool,
    words: bool,
    chars: bool,
}

impl WC {
    pub fn new(
        file_path: Option<PathBuf>,
        bytes: bool,
        lines: bool,
        words: bool,
        chars: bool,
    ) -> Self {
        let mut buf = BytesMut::new();
        let mut temp = Vec::new();
        match &file_path {
            Some(path) => {
                let mut file = File::open(path).unwrap();
                file.read_to_end(&mut temp).unwrap();
            }
            None => {
                io::stdin().read_to_end(&mut temp).unwrap();
            }
        }
        buf.extend_from_slice(&temp);

        Self {
            buf,
            file_path,
            bytes,
            lines,
            words,
            chars,
        }
    }
    pub fn word_count(&self) -> usize {
        let s = String::from_utf8_lossy(&self.buf);
        s.split_whitespace().count()
    }

    pub fn lines_count(&self) -> usize {
        let mut lines_counter = 0;
        for b in &self.buf {
            if *b == b'\n' {
                lines_counter += 1;
            }
        }
        lines_counter
    }
    pub fn bytes_count(&self) -> usize {
        self.buf.len()
    }

    pub fn char_count(&self) -> usize {
        let s = String::from_utf8_lossy(&self.buf);
        s.chars().count()
    }

    pub fn execute(&self) {
        let file_name = match &self.file_path {
            Some(file) => file.to_str().unwrap(),
            None => "",
        };

        if !self.bytes && !self.chars && !self.words && !self.lines {
            println!(
                "  {} {} {} {}",
                self.lines_count(),
                self.word_count(),
                self.char_count(),
                file_name
            )
        } else {
            let mut output = String::new();
            if self.bytes {
                output += &format!("{}  ", self.bytes_count());
            }
            if self.lines {
                output += &format!("{}  ", self.lines_count());
            }
            if self.words {
                output += &format!("{}  ", self.word_count());
            }
            if self.chars {
                output += &format!("{}  ", self.char_count());
            }
            if !output.is_empty() {
                println!("  {}    {}", output, file_name)
            }
        }
    }
}
