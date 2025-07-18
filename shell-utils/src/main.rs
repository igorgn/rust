mod commands;
mod errors;
mod mapper;

use std::{
    env,
    error::Error,
    io::{self, Write},
};

use clap::{Command, Parser};

use mapper::map_command;

#[derive(Parser, Debug)]
struct Cli {
    program: String,
    args: Vec<String>,
}

fn setup_prompt() -> Result<String, io::Error> {
    let prompt = format!(
        "$_ {}",
        env::current_dir()?.file_name().unwrap().to_string_lossy()
    );
    print!("{} ", prompt);
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input)
}
fn main() -> Result<(), Box<dyn Error>> {
    loop {
        // TODO Implement Command interface
        let cmd = Command::new("Shell Utils");
        let input = setup_prompt()?;

        if input.trim().is_empty() {
            continue;
        }

        let args: Vec<String> = input
            .trim()
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();

        let cli = Cli {
            program: args[0].clone(),
            args: args[1..].to_vec()
        };
        // let cli = Cli::parse_from(args);

        let program = map_command(&cli.program);

        if let Err(e) = program.execute(cli.args) {
            println!("{}", e)
        }
    }
}
