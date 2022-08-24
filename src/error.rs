use std::fmt;

#[derive(Debug)]
pub enum Error {
    ParserError(String),
    RuntimeError(String),
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::ParserError(msg) => write!(f, "ParserError: {}", *msg),
            Error::RuntimeError(msg) => write!(f, "RuntimeError: {}", *msg)
        }
    }
}