mod cli;
mod cli_actions;
mod db;
mod errors;
mod helpers;
mod manager;
mod mapper;
mod web;
mod cli_args;

use crate::cli_args::start;

#[macro_use]
extern crate rocket;


#[rocket::main]
async fn main() {
    start().await;
}
