use std::fmt::{Display, Formatter};
use deducnat_macro::EnumDoc;
use crate::interpreter::InterpreterCommand;
use crate::repl::error::{Error, ReplError};

static COMMANDS: [&str; 5] = [
    "context",
    "help",
    "undo",
    "exit",
    "quit"
];

#[derive(Clone, PartialEq, EnumDoc)]
pub enum ReplCommand {
    #[cmd(name="context", usage="<name>", desc="Create a new proof context")]
    Context(String),
    #[cmd(name="help", desc="Display this information screen")]
    Help,
    #[cmd(name="help", usage="[command]", desc="Display information about a particular command")]
    HelpCommand(String),
    #[cmd(name="undo", desc="Revert last command while in proof mode")]
    Undo,
    #[cmd(name="exit", desc="Close sub-screens (help, list) or go back to main screen")]
    Exit,
    #[cmd(name="quit", desc="Stop deducnat")]
    Quit,
    Return // Emitted when inputting an empty field
}

impl Display for ReplCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ReplCommand::Context(s) => write!(f, "context {s}"),
            ReplCommand::Help => write!(f, "help"),
            ReplCommand::HelpCommand(s) => write!(f, "help {s}"),
            ReplCommand::Undo => write!(f, "undo"),
            ReplCommand::Exit => write!(f, "exit"),
            ReplCommand::Quit => write!(f, "quit"),
            ReplCommand::Return => write!(f, ""),
        }
    }
}

#[derive(PartialEq, Clone)]
pub enum Command {
    InterpreterCommand(InterpreterCommand),
    ReplCommand(ReplCommand)
}

impl Command {
    pub fn name(&self) -> Option<String> {
        match self {
            Command::InterpreterCommand(c) => c.name(),
            Command::ReplCommand(c) => c.name()
        }
    }
    pub fn desc(&self) -> Option<String> {
        match self {
            Command::InterpreterCommand(c) => c.desc(),
            Command::ReplCommand(c) => c.desc()
        }
    }
    pub fn usage(&self) -> Option<String> {
        match self {
            Command::InterpreterCommand(c) => c.usage(),
            Command::ReplCommand(c) => c.usage()
        }
    }
    pub fn schema(&self) -> Option<(Vec<String>, String)> {
        match self {
            Command::InterpreterCommand(InterpreterCommand::RuleCommand(r)) => r.schema(),
            _ => None
        }
    }


    pub fn from(command_str: &str) -> Result<Command, Error> {
        let command_str = command_str.trim();
        if command_str.is_empty() {return Ok(Command::ReplCommand(ReplCommand::Return))}

        // a command is made up of a command name and command arguments.
        let (cname, cparam) = command_str.split_once(' ').unwrap_or_else(|| (command_str, ""));
        let cparam = cparam.to_string();

        let command = match (cname, cparam) {
            ("context", s) if !s.is_empty() => Command::ReplCommand(ReplCommand::Context(s)),
            ("help", s) if s.is_empty() => Command::ReplCommand(ReplCommand::Help),
            ("help", s) if !s.is_empty() => Command::ReplCommand(ReplCommand::HelpCommand(s)),
            ("undo", s) if s.is_empty() => Command::ReplCommand(ReplCommand::Undo),
            ("exit", s) if s.is_empty() => Command::ReplCommand(ReplCommand::Exit),
            ("quit", s) if s.is_empty() => Command::ReplCommand(ReplCommand::Quit),

            (e, _) => {
                if COMMANDS.contains(&e) {
                    return Err(Error::ReplError(ReplError::InvalidCommand(e.to_string())))
                }

                match InterpreterCommand::from(command_str) {
                    Ok(cmd) => Command::InterpreterCommand(cmd),
                    Err(e) => {
                        return Err(Error::InterpreterError(e))
                    }
                }
            }
        };

        Ok(command)
    }
}


impl Display for Command {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Command::InterpreterCommand(c) => c.fmt(f),
            Command::ReplCommand(c) => c.fmt(f)
        }
    }
}