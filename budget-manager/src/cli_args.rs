use clap::Parser;

use crate::{cli::run_cli, web};

#[derive(Parser)]
pub struct CliArgs {
    #[arg(long, group = "mode")]
    web: bool,
    #[arg(long, group = "mode")]
    cli: bool,
}

pub async fn start() {
    let args = CliArgs::parse();
    if args.web {
        let rocket = web::build_rocket();
        if let Err(e) = rocket.launch().await {
            eprintln!("{}", e);
        }
    } else if args.cli {
        if let Err(e) = run_cli() {
            eprintln!("{}", e)
        }
    }
}