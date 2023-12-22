use std::fmt::{Display, Formatter};
use strum::EnumIter;
use sculpt_macro::{EnumDoc, EnumType};
use crate::{logic::rule::{Rule, RuleType, Side}, error::Error};


static COMMANDS: [&str; 19] = [
    //"context",
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


/// Control command for the interpreter. Those rules are not directly linked to natural deduction.
#[derive(Clone, Debug, EnumIter, EnumDoc, EnumType, PartialEq)]
pub enum EngineCommand {

    #[cmd(name="Thm", usage="<thm_name> :: <F>", desc="Create a new theorem and start the proof mode")]
    Theorem(String, String),
    #[cmd(name="admit", desc="Consider the current goal proven, exit proof mode")]
    Admit,
    #[cmd(name="use", usage="use <thm_name>", desc="Adds a theorem to the proof context")]
    Use(String),
    #[cmd(name="qed", desc="Finish the proof & exit proof mode (only when no more subgoals)")]
    Qed
}

impl Display for EngineCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            //InterpreterCommand::Context(s) => write!(f, "context {s}"),
            EngineCommand::Theorem(name, formula) => write!(f, "Thm {name} :: {formula}"),
            EngineCommand::Admit => write!(f, "admit"),
            e => match e.name() {
                Some(n) => write!(f, "{n}"),
                None => Ok(())
            }
        }
    }
}



/// Command only available during a proof. Applies natural deduction rules.
#[derive(Clone, Debug, EnumIter, EnumDoc, EnumType, PartialEq)]
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
    #[cmd(name="from_or", usage="<F> \\/ <G>")]
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
            RuleCommand::FromOr(_) => (vec!["Γ ⊢ F \\/ G", "Γ, F ⊢ H", "Γ, G ⊢ H"], "Γ ⊢ H"),
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


    pub fn to_rule(self) -> Rule {
        match self {
            RuleCommand::Axiom => Rule::Axiom,
            RuleCommand::Intro => Rule::Intro,
            RuleCommand::Intros => Rule::Intros,
            RuleCommand::Trans(s) => Rule::Trans(s),
            RuleCommand::Split => Rule::SplitAnd,
            RuleCommand::AndLeft(s) => Rule::And(Side::Left, s),
            RuleCommand::AndRight(s) => Rule::And(Side::Right, s),
            RuleCommand::KeepLeft => Rule::Keep(Side::Left),
            RuleCommand::KeepRight => Rule::Keep(Side::Right),
            RuleCommand::FromOr(s) => Rule::FromOr(s),
            RuleCommand::Generalize(s) => Rule::Generalize(s),
            RuleCommand::FixAs(s) => Rule::FixAs(s),
            RuleCommand::Consider(s) => Rule::Consider(s),
            RuleCommand::RenameAs(s) => Rule::RenameAs(s),
            RuleCommand::FromBottom => Rule::FromBottom,
            RuleCommand::ExFalso(s) => Rule::ExFalso(s)
        }
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



#[derive(Clone, PartialEq, Debug)]
pub enum InterpreterCommand {
    EngineCommand(EngineCommand),
    RuleCommand(RuleCommand)
}

impl Display for InterpreterCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            InterpreterCommand::EngineCommand(c) => c.fmt(f),
            InterpreterCommand::RuleCommand(c) => c.fmt(f),
        }
    }
}


impl InterpreterCommand {
    /// Creates a command from a string. Typically, this string will either be a line from a file,
    /// or the command read by a REPL.
    pub fn from(command_str: &str) -> Result<InterpreterCommand, Error> {
        let command_str = command_str.trim();
        if command_str.is_empty() {return Err(Error::EmptyCommand)}

        // a command is made up of a command name and command arguments.
        let (cname, cparam) = command_str.split_once(' ').unwrap_or_else(|| (command_str, ""));
        let cparam = cparam.to_string();


        todo!()
        //let command = match (cname, cparam) {

            // Interpreter commands
            //("context", s) if s.len() > 0 => Command::Interpreter(InterpreterCommand::Context(s)),
            /*("admit", s) if !s.is_empty() => InterpreterCommand::EngineCommand(EngineCommand::Admit),
            ("admit", s) if s.is_empty() => {
                return Err(Error::ArgumentsRequired("Expected a formula".to_string()))
            }

            ("proof", s) if !s.is_empty() => InterpreterCommand::EngineCommand(EngineCommand::Theorem(s)),
            ("proof", s) if s.is_empty() => {
                return Err(Error::ArgumentsRequired("Expected a formula".to_string()))
            }

            ("qed", s) if s.len() == 0 => InterpreterCommand::EngineCommand(EngineCommand::Qed),

            // Rule commands
            ("axiom", s) if s.len() == 0 => InterpreterCommand::RuleCommand(RuleCommand::Axiom),
            ("intro", s) if s.len() == 0 => InterpreterCommand::RuleCommand(RuleCommand::Intro),
            ("intros", s) if s.len() == 0 => InterpreterCommand::RuleCommand(RuleCommand::Intros),
            ("trans", s) => InterpreterCommand::RuleCommand(RuleCommand::Trans(s)),
            ("split", s) if s.len() == 0 => InterpreterCommand::RuleCommand(RuleCommand::Split),
            ("and_left", s) => InterpreterCommand::RuleCommand(RuleCommand::AndLeft(s)),
            ("and_right", s) => InterpreterCommand::RuleCommand(RuleCommand::AndRight(s)),
            ("keep_left", s) if s.len() == 0 => InterpreterCommand::RuleCommand(RuleCommand::KeepLeft),
            ("keep_right", s) if s.len() == 0 => InterpreterCommand::RuleCommand(RuleCommand::KeepRight),
            ("from_or", s) => InterpreterCommand::RuleCommand(RuleCommand::FromOr(s)),
            ("gen", s) => InterpreterCommand::RuleCommand(RuleCommand::Generalize(s)),
            ("fix_as", s) => InterpreterCommand::RuleCommand(RuleCommand::FixAs(s)),
            ("consider", s) => InterpreterCommand::RuleCommand(RuleCommand::Consider(s)),
            ("rename_as", s) => InterpreterCommand::RuleCommand(RuleCommand::RenameAs(s)),
            ("from_bottom", s) if s.len() == 0 => InterpreterCommand::RuleCommand(RuleCommand::FromBottom),
            ("exfalso", s) => InterpreterCommand::RuleCommand(RuleCommand::ExFalso(s)),

            ("qed" | "axiom" | "intro" | "intros" | "spit" | "keep_left" | "keep_right" | "from_bottom", s) if s.len() > 0 => {
                return Err(Error::TooMuchArguments(cname.to_string()))
            }

            (cn, _) => {
                if COMMANDS.contains(&cn) {
                    return Err(Error::InvalidCommand(cn.to_string()))
                }
                return Err(Error::UnknownCommand(cn.to_string()))
            }*/
        //};

        //Ok(command)
    }



    pub fn name(&self) -> Option<String> {
        match self {
            InterpreterCommand::EngineCommand(c) => c.name(),
            InterpreterCommand::RuleCommand(c) => c.name(),
        }
    }
    pub fn desc(&self) -> Option<String> {
        match self {
            InterpreterCommand::EngineCommand(c) => c.desc(),
            InterpreterCommand::RuleCommand(c) => c.desc(),
        }
    }
    pub fn usage(&self) -> Option<String> {
        match self {
            InterpreterCommand::EngineCommand(c) => c.usage(),
            InterpreterCommand::RuleCommand(c) => c.usage(),
        }
    }
}
