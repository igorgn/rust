use std::io::{self, Read};

use anyhow::Result;

pub mod errors;
mod lexer;
mod parser;
mod consts;

// impl Jq {
//     pub fn new(file_path: PathBuf) -> Self {
//         let mut buf = String::new();
//         let mut file = File::open(&file_path).unwrap();
//         file.read_to_string(&mut buf).unwrap();

//         Self {
//             json_raw: buf.trim().to_string(),
//         }
//     }

//     pub fn parse(&self) -> Result<()> {
//         let mut lex = lexer::Lexer::new(&self.json_raw);
//         while !lex.is_at_end() {
//             if let Ok(token) = lex.next_token() {
//                 println!("{:?}", token)
//             }
//         }
//         Ok(())
//     }
// }

fn main() -> Result<()> {
    let mut buf = String::new();

    io::stdin().read_to_string(&mut buf)?;

    let mut lex = lexer::Lexer::new(&buf);
    while !lex.is_at_end() {
        if let Ok(token) = lex.next_token() {
            println!("{:?}", token)
        }
    }

    Ok(())
}
