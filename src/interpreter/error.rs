use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub enum InterpretorError {
    InvalidArguments(String),
    UnknownCommand(String),
    InvalidCommand(String),
    CommandError(String),
    EmptyCommand,
}

impl Display for InterpretorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InterpretorError::InvalidArguments(s) => write!(f, "invalid arguments for command '{s}', see command documentation for help"),
            InterpretorError::UnknownCommand(s) => write!(f, "unknown command '{s}'"),
            InterpretorError::InvalidCommand(s) => write!(f, "command '{s}' exists but is not valid in this context"),
            InterpretorError::CommandError(e) => write!(f, "{e}"),
            InterpretorError::EmptyCommand => write!(f, "empty command")
        }
    }
}