use thiserror::Error;

#[derive(Error, Debug)]
pub enum WebServerError {
    #[error("Parse error")]
    Parse(#[from] RequestParseError),
    #[error("Stream error")]
    StreamError,
}

#[derive(Error, Debug)]
pub enum RequestParseError {
    #[error("Missing method")]
    MissingMethod,
    #[error("Missing path")]
    MissingPath,
    #[error("Missing version")]
    MissingVersion,
    #[error("Unsupported version")]
    UnsupportedVersion,
    #[error("Invalid Method")]
    InvalidMethod,
}

#[derive(Error, Debug)]
pub enum ResponseBuildError {
    #[error("Response builder error")]
    RespBuildError,
}
