//! This module regroups structures used to interpret commands, either from the REPL or
//! from a file.
mod command;
mod interpreter;

pub use command::{InterpreterCommand, EngineCommand};
pub use interpreter::Interpreter;