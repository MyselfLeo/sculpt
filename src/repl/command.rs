use std::fmt::{Display, Formatter};
use sculpt_macro::EnumDoc;
use strum::EnumIter;
use crate::engine::EngineCommand;
use crate::error::Error;
use crate::syntax::lexer::{Context, Lexer};

static COMMANDS: [&str; 6] = [
    "context",
    "help",
    "list",
    "undo",
    "exit",
    "quit"
];

#[derive(Clone, PartialEq, EnumDoc, EnumIter)]
pub enum ReplCommand {
    #[cmd(name="context", usage="<name>", desc="Create a new proof context")]
    Context(String),
    #[cmd(name="help", desc="Display this information screen")]
    Help,
    #[cmd(name="help", usage="[command]", desc="Display information about a particular command")]
    HelpCommand(String),
    #[cmd(name="list", desc="Display the list of commands for the current context")]
    List,
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
            ReplCommand::List => write!(f, "list"),
            ReplCommand::Undo => write!(f, "undo"),
            ReplCommand::Exit => write!(f, "exit"),
            ReplCommand::Quit => write!(f, "quit"),
            ReplCommand::Return => write!(f, ""),
        }
    }
}

#[derive(PartialEq, Clone)]
pub enum Command {
    EngineCommand(EngineCommand),
    ReplCommand(ReplCommand)
}

impl Command {
    pub fn name(&self) -> Option<String> {
        match self {
            Command::EngineCommand(c) => c.name(),
            Command::ReplCommand(c) => c.name()
        }
    }
    pub fn desc(&self) -> Option<String> {
        match self {
            Command::EngineCommand(c) => c.desc(),
            Command::ReplCommand(c) => c.desc()
        }
    }
    pub fn usage(&self) -> Option<String> {
        match self {
            Command::EngineCommand(c) => c.usage(),
            Command::ReplCommand(c) => c.usage()
        }
    }
    pub fn schema(&self) -> Option<(Vec<String>, String)> {
        match self {
            Command::EngineCommand(EngineCommand::RuleCommand(r)) => r.schema(),
            _ => None
        }
    }


    pub fn from(command_str: &str, context: &Context) -> Result<Command, Error> {
        let command_str = command_str.trim();
        if command_str.is_empty() {return Ok(Command::ReplCommand(ReplCommand::Return))}


        // Try to parse REPL-specific commands. If it fails, try to parse an engine command otherwise.
        let (cname, cparam) = command_str.split_once(' ').unwrap_or_else(|| (command_str, ""));
        let cparam = cparam.to_string();
        let command = match (cname, cparam) {
            ("context", s) if !s.is_empty() => Command::ReplCommand(ReplCommand::Context(s)),
            ("context", s) if s.is_empty() => {
                return Err(Error::ArgumentsRequired("Expected a context name".to_string()))
            }

            ("help", s) if s.is_empty() => Command::ReplCommand(ReplCommand::Help),
            ("help", s) if !s.is_empty() => Command::ReplCommand(ReplCommand::HelpCommand(s)),

            ("list", s) if s.is_empty() => Command::ReplCommand(ReplCommand::List),
            ("list", s) if !s.is_empty() => {
                return Err(Error::TooMuchArguments(cname.to_string()))
            }

            ("undo", s) if s.is_empty() => Command::ReplCommand(ReplCommand::Undo),
            ("undo", s) if !s.is_empty() => {
                return Err(Error::TooMuchArguments(cname.to_string()))
            }

            ("exit", s) if s.is_empty() => Command::ReplCommand(ReplCommand::Exit),
            ("exit", s) if !s.is_empty() => {
                return Err(Error::TooMuchArguments(cname.to_string()))
            }
            ("quit", s) if s.is_empty() => Command::ReplCommand(ReplCommand::Quit),
            ("quit", s) if !s.is_empty() => {
                return Err(Error::TooMuchArguments(cname.to_string()))
            }

            _ => {
                match EngineCommand::parse(&mut Lexer::from(command_str, context.clone()))? {
                    None => Command::ReplCommand(ReplCommand::Return), // empty command, probably a comment
                    Some(c) => Command::EngineCommand(c)
                }
            }
        };

        Ok(command)
    }
}


impl Display for Command {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Command::EngineCommand(c) => c.fmt(f),
            Command::ReplCommand(c) => c.fmt(f)
        }
    }
}