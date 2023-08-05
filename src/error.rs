use core::fmt;
use std::fmt::Formatter;

#[derive(Debug)]
pub struct Error {
    error_msg: String
}

impl Error {
    pub fn new(error_msg: String) -> Self {
        Error { error_msg }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Custom error: {}", self.error_msg)
    }
}

pub trait PrefixedError {
    fn get_prefix(&self) -> &str;

    fn error(&self, error_msg: &str) -> Error {
        Error::new(format!("{}: {}", self.get_prefix(), error_msg))
    }
}