use std::fmt::{Debug, Display, Formatter};
use crate::logic::{Formula, Term};

pub enum Error {
    CommandError(String),               // Error during command execution
    InvalidArguments(String),           // Valid command but incorrect arguments
    InvalidFormula(Formula, String),    // Problem with a formula
    InvalidTerm(Term, String),          // Problem with a formula
    InvalidCommand(String),             // Invalid command (in this context)
    TooMuchArguments(String),           // Arguments given but not expected
    ArgumentsRequired(String),          // No arguments given but arguments expected
    UnableToRead,                       // I/O error
    UnknownCommand(String),             // Unknown command
    EmptyFile(String),                  // Empty file
    UnfinishedProof,
    UnexpectedEOF,
    AlreadyExists(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::CommandError(e) => std::fmt::Display::fmt(e, f),
            Error::InvalidArguments(e) => std::fmt::Display::fmt(e, f),
            Error::InvalidCommand(c) => write!(f, "Command '{c}' exists but is not valid in this context"),
            Error::InvalidFormula(form, s) => write!(f, "Invalid formula '{form}': {s}"),
            Error::InvalidTerm(term, s) => write!(f, "Invalid term '{term}': {s}"),
            Error::TooMuchArguments(c) => write!(f, "Command '{c}' does not expect arguments"),
            Error::ArgumentsRequired(e) => std::fmt::Display::fmt(e, f),
            Error::UnableToRead => write!(f, "Unable to read input"),
            Error::UnknownCommand(c) => write!(f, "Command {c} does not exist"),
            Error::EmptyFile(name) => write!(f, "Empty file {name}"),
            Error::UnfinishedProof => write!(f, "Unfinished proof"),
            Error::UnexpectedEOF => write!(f, "Unexpected end-of-file. Have you forgot a '.' ?"),
            Error::AlreadyExists(sym) => write!(f, "{sym} already exists"),
        }
    }
}

impl Debug for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}