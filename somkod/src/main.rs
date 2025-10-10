mod wc;
use crate::wc::WC;
use anyhow::{Ok, Result};
use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(name = "somkod")]
#[command(about = "Playground for leetcode and small cli code challenges")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Wc {
        file_path: Option<PathBuf>,

        #[arg(short = 'c', long, default_value_t = false)]
        bytes: bool,

        #[arg(short = 'm', long, default_value_t = false)]
        chars: bool,

        #[arg(short, long, default_value_t = false)]
        lines: bool,

        #[arg(short, long, default_value_t = false)]
        words: bool,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Some(command) => match command {
            Commands::Wc {
                file_path,
                bytes,
                chars,
                lines,
                words,
            } => {
                let wc = WC::new(file_path, bytes, lines, words, chars);
                wc.execute();
            }
        },
        None => {}
    }
    Ok(())
}
