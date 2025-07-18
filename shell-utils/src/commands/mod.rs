use std::{error::Error, process::exit};

use crate::errors::CliErrors;

mod cat;
mod echo;
mod grep;
mod ls;

pub enum Commands {
    Ls,
    Cat,
    Echo,
    Exit,
    Grep,
    Unknown(String),
}

impl Commands {
    pub fn execute(&self, args: Vec<String>) -> Result<(), Box<dyn Error>> {
        match self {
            Commands::Ls => ls::execute(args),
            Commands::Cat => cat::execute(args),
            Commands::Echo => echo::execute(args),
            Commands::Grep => grep::execute(args, &mut std::io::stdout()),
            Commands::Exit => exit(0),
            Commands::Unknown(e) => Err(Box::new(CliErrors::CommandNotFound(e.to_string()))),
        }
    }
}
