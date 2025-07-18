use thiserror::Error;

#[derive(Error, Debug)]
pub enum CliErrors {
    #[error("Command not found: {0}")]
    CommandNotFound(String),
}
