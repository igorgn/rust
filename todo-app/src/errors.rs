use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppErrors {
    #[error("Id {0} not found")]
    IdNotFound(String),
}
