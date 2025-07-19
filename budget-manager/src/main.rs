mod cli;
mod cli_actions;
mod cli_args;
mod db;
mod errors;
mod helpers;
mod manager;
mod mapper;
mod web;

use crate::cli_args::start;

#[macro_use]
extern crate rocket;

#[rocket::main]
async fn main() {
    start().await;
}
