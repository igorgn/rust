use thiserror::Error;

#[derive(Debug, Error)]
pub enum LexError {
    #[error("Unexpected EOF")]
    UnexpectedEOF,
    #[error("Unexpected character")]
    UnexpectedChar,
    #[error("Failed to parse a number")]
    FailedNumberParse,
}
