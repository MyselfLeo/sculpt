//! This module regroups structures used to interpret commands, either from the REPL or
//! from a file.
mod command;
mod error;
mod interpreter;

pub use error::InterpretorError;
pub use command::{Command, InterpreterCommand, RuleCommand};