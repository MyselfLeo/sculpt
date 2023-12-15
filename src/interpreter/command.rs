use std::fmt::{Display, Formatter};
use strum::EnumIter;
use deducnat_macro::{EnumDoc, EnumType};
use crate::logic::rule::RuleType;
use super::InterpretorError;


static COMMANDS: [&str; 20] = [
    "context",
    "admit",
    "proof",
    "qed",
    "axiom",
    "intro",
    "intros",
    "trans",
    "split",
    "and_left",
    "and_right",
    "keep_left",
    "keep_right",
    "from_or",
    "gen",
    "fix_as",
    "consider",
    "rename_as",
    "from_bottom",
    "exfalso"
];


/// Control command for the interpreter. Create context, start proof, finish proof, etc.
#[derive(Clone, EnumIter, EnumDoc, EnumType)]
pub enum InterpreterCommand {
    #[cmd(name="context", usage="<name>", desc="Create a new proof context")]
    Context(String),
    #[cmd(name="proof", usage="<F>", desc="Start the proving process of F in the current context")]
    Proof(String),
    #[cmd(name="admit", usage="<F>", desc="Add an unproven assumption to the current context")]
    Admit(String),
    #[cmd(name="qed", desc="Finish the proof (only when no more subgoals)")]
    Qed
}

impl Display for InterpreterCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            InterpreterCommand::Context(s) => write!(f, "context {s}"),
            InterpreterCommand::Proof(s) => write!(f, "proof {s}"),
            InterpreterCommand::Admit(s) => write!(f, "admit {s}"),
            e => match e.name() {
                Some(n) => write!(f, "{n}"),
                None => Ok(())
            }
        }
    }
}



/// Command only available during a proof. Applies natural deduction rules.
#[derive(Clone, EnumIter, EnumDoc, EnumType)]
pub enum RuleCommand {
    #[cmd(name="axiom")]
    Axiom,
    #[cmd(name="intro")]
    Intro,
    #[cmd(name="intros", desc="Apply multiple 'intro' rules, until it's not longer possible")]
    Intros,
    #[cmd(name="trans", usage="<F>")]
    Trans(String),
    #[cmd(name="split")]
    Split,
    #[cmd(name="and_left", usage="<F>")]
    AndLeft(String),
    #[cmd(name="and_right", usage="<F>")]
    AndRight(String),
    #[cmd(name="keep_left")]
    KeepLeft,
    #[cmd(name="keep_right")]
    KeepRight,
    #[cmd(name="from_or", usage="<F1> \\/ <F2>")]
    FromOr(String),
    #[cmd(name="gen", usage="<T>")]
    Generalize(String),
    #[cmd(name="fix_as", usage="<T>")]
    FixAs(String),
    #[cmd(name="consider", usage="exists <v>, <F>")]
    Consider(String),
    #[cmd(name="rename_as", usage="<v>")]
    RenameAs(String),
    #[cmd(name="from_bottom", usage="<F>")]
    FromBottom,
    #[cmd(name="exfalso")]
    ExFalso(String)
}


impl Display for RuleCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            RuleCommand::Trans(s) => write!(f, "trans {s}"),
            RuleCommand::AndLeft(s) => write!(f, "and_left {s}"),
            RuleCommand::AndRight(s) => write!(f, "and_right {s}"),
            RuleCommand::FromOr(s) => write!(f, "from_or {s}"),
            RuleCommand::Generalize(s) => write!(f, "gen {s}"),
            RuleCommand::FixAs(s) => write!(f, "fix_as {s}"),
            RuleCommand::Consider(s) => write!(f, "consider {s}"),
            RuleCommand::RenameAs(s) => write!(f, "rename_as {s}"),
            RuleCommand::ExFalso(s) => write!(f, "exfalso {s}"),
            e => match e.name() {
                Some(n) => write!(f, "{n}"),
                None => Ok(())
            }
        }
    }
}


impl RuleCommand {
    /// Returns the schema associated to the rule.
    pub fn schema(&self) -> Option<(Vec<String>, String)> {
        let (ante, cons) = match self {
            RuleCommand::Axiom => (vec![""], "Γ, F ⊢ F"),
            RuleCommand::Intro => (vec!["Γ, F ⊢ G"], "Γ ⊢ F => G"),
            RuleCommand::Trans(_) => (vec!["Γ ⊢ F => G", "Γ ⊢ F"], "Γ ⊢ G"),
            RuleCommand::Split => (vec!["Γ ⊢ F", "Γ ⊢ G"], "Γ ⊢ F /\\ G"),
            RuleCommand::AndLeft(_) => (vec!["Γ ⊢ F /\\ G"], "Γ ⊢ G"),
            RuleCommand::AndRight(_) => (vec!["Γ ⊢ G /\\ F"], "Γ ⊢ G"),
            RuleCommand::KeepLeft => (vec!["Γ ⊢ F"], "Γ ⊢ F \\/ G"),
            RuleCommand::KeepRight => (vec!["Γ ⊢ G"], "Γ ⊢ F \\/ G"),
            RuleCommand::FromOr(_) => (vec!["Γ ⊢ F1 \\/ F2", "Γ, F1 ⊢ H", "Γ, F2 ⊢ H"], "Γ ⊢ H"),
            RuleCommand::Generalize(_) => (vec!["Γ ⊢ forall v, F"], "Γ ⊢ F[v -> T]"),
            RuleCommand::FixAs(_) => (vec!["Γ ⊢ F[v -> T]"], "Γ ⊢ exists v, F"),
            RuleCommand::Consider(_) => (vec!["Γ ⊢ exists v, F", "Γ, F ⊢ G"], "Γ ⊢ G"),
            RuleCommand::RenameAs(_) => (vec!["Γ ⊢ forall/exists v, F[x -> v]"], "Γ ⊢ forall/exists x, F"),
            RuleCommand::FromBottom => (vec!["Γ, ~F ⊢ falsum"], "Γ ⊢ F"),
            RuleCommand::ExFalso(_) => (vec!["Γ ⊢ F", "Γ ⊢ ~F"], "Γ ⊢ falsum"),

            _ => return None
        };
        let ante_str: Vec<_> = ante.iter().map(|s| s.to_string()).collect();
        Some((ante_str, cons.to_string()))
    }
}


impl RuleCommandType {
    /// Return a list of rule command type based on the rule type it can generate.
    pub fn from_rule(rule: &RuleType) -> Vec<RuleCommandType> {
        match rule {
            RuleType::Axiom => vec![RuleCommandType::Axiom],
            RuleType::Intro => vec![RuleCommandType::Intro],
            RuleType::Intros => vec![RuleCommandType::Intros],
            RuleType::Trans => vec![RuleCommandType::Trans],
            RuleType::SplitAnd => vec![RuleCommandType::Split],
            RuleType::And => vec![RuleCommandType::AndRight, RuleCommandType::AndLeft],
            RuleType::Keep => vec![RuleCommandType::KeepRight, RuleCommandType::KeepLeft],
            RuleType::FromOr => vec![RuleCommandType::FromOr],
            RuleType::Generalize => vec![RuleCommandType::Generalize],
            RuleType::FixAs => vec![RuleCommandType::FixAs],
            RuleType::Consider => vec![RuleCommandType::Consider],
            RuleType::RenameAs => vec![RuleCommandType::RenameAs],
            RuleType::FromBottom => vec![RuleCommandType::FromBottom],
            RuleType::ExFalso => vec![RuleCommandType::ExFalso],
        }
    }
}




pub enum Command {
    Interpreter(InterpreterCommand),
    Rule(RuleCommand)
}

impl Display for Command {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Command::Interpreter(c) => c.fmt(f),
            Command::Rule(c) => c.fmt(f),
        }
    }
}


impl Command {
    /// Creates a command from a string. Typically, this string will either be a line from a file,
    /// or the command read by a REPL.
    pub fn from(command_str: &str) -> Result<Command, InterpretorError> {
        let command_str = command_str.trim();
        if command_str.is_empty() {return Err(InterpretorError::EmptyCommand)}

        // a command is made up of a command name and command arguments.
        let (cname, cparam) = command_str.split_once(' ').unwrap_or_else(|| (command_str, ""));
        let cparam = cparam.to_string();

        let command = match (cname, cparam) {

            // Interpreter commands
            ("context", s) if s.len() > 0 => Command::Interpreter(InterpreterCommand::Context(s)),
            ("admit", s) if s.len() > 0 => Command::Interpreter(InterpreterCommand::Admit(s)),
            ("proof", s) if s.len() > 0 => Command::Interpreter(InterpreterCommand::Proof(s)),
            ("qed", s) if s.len() == 0 => Command::Interpreter(InterpreterCommand::Qed),

            // Rule commands
            ("axiom", s) if s.len() == 0 => Command::Rule(RuleCommand::Axiom),
            ("intro", s) if s.len() == 0 => Command::Rule(RuleCommand::Intro),
            ("intros", s) if s.len() == 0 => Command::Rule(RuleCommand::Intros),
            ("trans", s) if s.len() > 0 => Command::Rule(RuleCommand::Trans(s)),
            ("split", s) if s.len() == 0 => Command::Rule(RuleCommand::Split),
            ("and_left", s) if s.len() > 0 => Command::Rule(RuleCommand::AndLeft(s)),
            ("and_right", s) if s.len() > 0 => Command::Rule(RuleCommand::AndRight(s)),
            ("keep_left", s) if s.len() == 0 => Command::Rule(RuleCommand::KeepLeft),
            ("keep_right", s) if s.len() == 0 => Command::Rule(RuleCommand::KeepRight),
            ("from_or", s) if s.len() > 0 => Command::Rule(RuleCommand::FromOr(s)),
            ("gen", s) if s.len() > 0 => Command::Rule(RuleCommand::Generalize(s)),
            ("fix_as", s) if s.len() > 0 => Command::Rule(RuleCommand::FixAs(s)),
            ("consider", s) if s.len() > 0 => Command::Rule(RuleCommand::Consider(s)),
            ("rename_as", s) if s.len() > 0 => Command::Rule(RuleCommand::RenameAs(s)),
            ("from_bottom", s) if s.len() == 0 => Command::Rule(RuleCommand::FromBottom),
            ("exfalso", s) if s.len() > 0 => Command::Rule(RuleCommand::ExFalso(s)),

            (cn, _) => {
                if COMMANDS.contains(&cn) {
                    return Err(InterpretorError::InvalidCommand(cn.to_string()))
                }
                return Err(InterpretorError::UnknownCommand(cn.to_string()))
            }
        };

        Ok(command)
    }
}
