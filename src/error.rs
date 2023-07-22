use core::fmt;
use std::fmt::Formatter;

#[derive(Debug)]
pub struct Error(String);

impl Error {
   pub fn new(error_msg: String) -> Error {
       Error(error_msg)
   }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Error(s) => write!(f, "Custom error: {}", s)
        }
    }
}

pub trait PrefixedError {
    fn get_prefix(&self) -> &str;

    fn error(&self, error_msg: &str) -> Error {
        Error(format!("{}: {}", self.get_prefix(), error_msg))
    }
}