use std::fmt::{Display, Formatter};
use crate::interpreter::InterpretorError;

pub enum ReplError {
    EmptyCommand,
    InvalidCommand(String),
    UnableToRead
}

impl Display for ReplError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ReplError::InvalidCommand(s) => write!(f, "command '{s}' exists but is not valid in this context"),
            ReplError::EmptyCommand => write!(f, "empty command"),
            ReplError::UnableToRead => write!(f, "unable to read standart input")
        }
    }
}



pub enum Error {
    InterpreterError(InterpretorError),
    ReplError(ReplError)
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InterpreterError(e) => e.fmt(f),
            Error::ReplError(e) => e.fmt(f)
        }
    }
}