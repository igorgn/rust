mod cli;
mod cli_actions;
mod db;
mod errors;
mod helpers;
mod manager;
mod mapper;

use cli::run_cli;
fn main() {
    if let Err(e) = run_cli() {
        eprintln!("{}", e)
    }
}
