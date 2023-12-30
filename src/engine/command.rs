use std::fmt::{Debug, Display, Formatter};
use strum::EnumIter;
use sculpt_macro::{EnumDoc, EnumType};
use crate::{logic::rule::{Rule, RuleType, Side}, error::Error};
use crate::logic::{Formula, Term};
use crate::syntax::lexer::{Lexer, Token};





const DEFAULT_RULES: [&str; 19] = [
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

fn is_rule(str: &str) -> bool {
    DEFAULT_RULES.contains(&str)
}


/// Control command for the engine. Those rules are not directly linked to natural deduction.
#[derive(Clone, Debug, EnumIter, EnumDoc, EnumType, PartialEq)]
pub enum ContextCommand {

    #[cmd(name="Thm", usage="<thm_name> :: <F>", desc="Create a new theorem and start the proof mode")]
    Theorem(String, Box<Formula>),
    #[cmd(name="Use", usage="<thm_name>", desc="Adds a theorem to the proof context")]
    Use(String),
    #[cmd(name="Admit", desc="Consider the current goal proven, exit proof mode")]
    Admit,
    #[cmd(name="Qed", desc="Finish the proof & exit proof mode (only when no more subgoals)")]
    Qed,
}

impl Display for ContextCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            //InterpreterCommand::Context(s) => write!(f, "context {s}"),
            ContextCommand::Theorem(name, formula) => write!(f, "Thm {name} :: {formula}"),
            ContextCommand::Admit => write!(f, "admit"),
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
    Trans(Box<Formula>),
    #[cmd(name="split")]
    Split,
    #[cmd(name="and_left", usage="<F>")]
    AndLeft(Box<Formula>),
    #[cmd(name="and_right", usage="<F>")]
    AndRight(Box<Formula>),
    #[cmd(name="keep_left")]
    KeepLeft,
    #[cmd(name="keep_right")]
    KeepRight,
    #[cmd(name="from_or", usage="<F> \\/ <G>")]
    FromOr(Box<Formula>),
    #[cmd(name="gen", usage="<T>")]
    Generalize(Box<Term>),
    #[cmd(name="fix_as", usage="<T>")]
    FixAs(Box<Term>),
    #[cmd(name="consider", usage="exists <v>, <F>")]
    Consider(Box<Formula>),
    #[cmd(name="rename_as", usage="<v>")]
    RenameAs(String),
    #[cmd(name="from_bottom", usage="<F>")]
    FromBottom,
    #[cmd(name="exfalso", usage="<F>")]
    ExFalso(Box<Formula>)
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


    pub fn to_rule(&self) -> Rule {
        match &self {
            RuleCommand::Axiom => Rule::Axiom,
            RuleCommand::Intro => Rule::Intro,
            RuleCommand::Intros => Rule::Intros,
            RuleCommand::Trans(s) => Rule::Trans(s.clone()),
            RuleCommand::Split => Rule::SplitAnd,
            RuleCommand::AndLeft(s) => Rule::And(Side::Left, s.clone()),
            RuleCommand::AndRight(s) => Rule::And(Side::Right, s.clone()),
            RuleCommand::KeepLeft => Rule::Keep(Side::Left),
            RuleCommand::KeepRight => Rule::Keep(Side::Right),
            RuleCommand::FromOr(s) => Rule::FromOr(s.clone()),
            RuleCommand::Generalize(s) => Rule::Generalize(s.clone()),
            RuleCommand::FixAs(s) => Rule::FixAs(s.clone()),
            RuleCommand::Consider(s) => Rule::Consider(s.clone()),
            RuleCommand::RenameAs(s) => Rule::RenameAs(s.clone()),
            RuleCommand::FromBottom => Rule::FromBottom,
            RuleCommand::ExFalso(s) => Rule::ExFalso(s.clone())
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
pub enum EngineCommand {
    ContextCommand(ContextCommand),
    RuleCommand(RuleCommand)
}

impl Display for EngineCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            EngineCommand::ContextCommand(c) => std::fmt::Display::fmt(c, f),
            EngineCommand::RuleCommand(c) => std::fmt::Display::fmt(c, f),
        }
    }
}


impl EngineCommand {

    pub fn parse(command: &mut Lexer) -> Result<Option<EngineCommand>, Error> {

        let next = match command.next() {
            Some(c) => {
                c.expect("LexicalError")
            },
            None => return Ok(None) // empty command
        };

        let res = match next {
            (_, Token::Thm, _) => EngineCommand::parse_thm(command),
            (_, Token::Def, _) => EngineCommand::parse_def(command),
            (_, Token::Use, _) => EngineCommand::parse_use(command),
            (_, Token::Qed, _) => EngineCommand::parse_qed(command),
            (_, Token::Admit, _) => EngineCommand::parse_admit(command),
            (_, Token::Ident(s), _) => {
                if is_rule(&s) {
                    EngineCommand::parse_rule(command, s)
                }
                else {
                    Err(Error::UnknownCommand(s))
                }
            }
            (_, t, _) => Err(Error::UnknownCommand(t.to_string()))
        }?;

        // The tokens should be fully consumed after parsing a command. Otherwise, this is a
        // syntactical error
        if command.next().is_some() {
            Err(Error::TooMuchArguments("Too much arguments where supplied".to_string()))
        }
        else {
            Ok(Some(res))
        }
    }


    fn parse_thm(lxr: &mut Lexer) -> Result<EngineCommand, Error> {
        // Next token is the theorem name
        let thm_name = match lxr.next_token().expect("LexicalError") { // todo: use something other than UnexpectedEOF
            Some(Token::Ident(s)) => s,
            Some(t) => return Err(Error::InvalidCommand(format!("Expected a name, got '{t}'"))),
            None => return Err(Error::UnexpectedEOF)
        };

        // Next is '::'
        match lxr.next_token().expect("LexicalError") { // todo: same
            Some(Token::DoubleColon) => (),
            Some(t) => return Err(Error::InvalidCommand(format!("Expected '::', got '{t}'"))),
            None => return Err(Error::UnexpectedEOF)
        };

        // Next is the theorem's formula
        let formula = Formula::parse(lxr)?;

        Ok(EngineCommand::ContextCommand(ContextCommand::Theorem(thm_name, Box::new(formula))))
    }


    fn parse_def(_: &mut Lexer) -> Result<EngineCommand, Error> {
        unimplemented!()
    }


    fn parse_use(lxr: &mut Lexer) -> Result<EngineCommand, Error> {
        match lxr.next() {
            None => Err(Error::ArgumentsRequired("Expected a theorem name".to_string())),
            Some(r) => match r.expect("LexicalError") {
                (_, Token::Ident(s), _) => Ok(EngineCommand::ContextCommand(ContextCommand::Use(s))),
                (_, t, _) => Err(Error::InvalidArguments(format!("Expected a theorem name, got '{t}")))
            }
        }
    }


    fn parse_qed(_: &mut Lexer) -> Result<EngineCommand, Error> {
        Ok(EngineCommand::ContextCommand(ContextCommand::Qed))
    }

    fn parse_admit(_: &mut Lexer) -> Result<EngineCommand, Error> {
        Ok(EngineCommand::ContextCommand(ContextCommand::Admit))
    }

    fn parse_rule(lxr: &mut Lexer, rule_name: String) -> Result<EngineCommand, Error> {
        let parse_formula = |lxr: &mut Lexer| -> Result<Box<Formula>, Error> {
            if lxr.is_finished() {
                return Err(Error::ArgumentsRequired("Expected a formula".to_string()))
            }
            Formula::parse(lxr).map(Box::new)
        };
        let parse_term = |lxr: &mut Lexer| -> Result<Box<Term>, Error> {
            if lxr.is_finished() {
                return Err(Error::ArgumentsRequired("Expected a term".to_string()))
            }
            Term::parse(lxr).map(Box::new)
        };

        let rc = match rule_name.as_str() {
            "axiom" => RuleCommand::Axiom,
            "intro" => RuleCommand::Intro,
            "intros" => RuleCommand::Intros,
            "trans" => RuleCommand::Trans(parse_formula(lxr)?),
            "split" => RuleCommand::Split,
            "and_left" => RuleCommand::AndLeft(parse_formula(lxr)?),
            "and_right" => RuleCommand::AndRight(parse_formula(lxr)?),
            "keep_left" => RuleCommand::KeepLeft,
            "keep_right" => RuleCommand::KeepRight,
            "from_or" => RuleCommand::FromOr(parse_formula(lxr)?),
            "gen" => RuleCommand::Generalize(parse_term(lxr)?),
            "fix_as" => RuleCommand::FixAs(parse_term(lxr)?),
            "consider" => RuleCommand::Consider(parse_formula(lxr)?),
            "rename_as" => {
                match lxr.next() {
                    Some(r) => match r {
                        Ok((_, Token::Ident(s), _)) => RuleCommand::RenameAs(s),
                        _ => return Err(Error::InvalidArguments("Expected a variable name".to_string()))
                    },
                    None => return Err(Error::ArgumentsRequired("Expected a variable name".to_string()))
                }
            },
            "from_bottom" => RuleCommand::FromBottom,
            "exfalso" => RuleCommand::ExFalso(parse_formula(lxr)?),
            _ => unreachable!(), // lexer should not generate a Token::RuleName if rule_name is not in this list
        };

        Ok(EngineCommand::RuleCommand(rc))
    }


    /*pub fn from(command_str: &str) -> Result<EngineCommand, Error> {
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
    }*/



    pub fn name(&self) -> Option<String> {
        match self {
            EngineCommand::ContextCommand(c) => c.name(),
            EngineCommand::RuleCommand(c) => c.name(),
        }
    }
    pub fn desc(&self) -> Option<String> {
        match self {
            EngineCommand::ContextCommand(c) => c.desc(),
            EngineCommand::RuleCommand(c) => c.desc(),
        }
    }
    pub fn usage(&self) -> Option<String> {
        match self {
            EngineCommand::ContextCommand(c) => c.usage(),
            EngineCommand::RuleCommand(c) => c.usage(),
        }
    }
}
