mod jq;
mod wc;

use crate::jq::Jq;
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

    Jq {
        file_path: PathBuf,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
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

            Commands::Jq { file_path } => {
                let jq = Jq::new(file_path);
                // jq.parse();
                if let Err(err) = jq.parse() {
                    eprintln!("{}", err);
                    std::process::exit(1)
                }
            }
        }
    }
    Ok(())
}
