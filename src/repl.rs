use std::cell::RefCell;
use std::cmp::max;
use std::fmt::Display;
use std::io;
use std::io::Write;
use crossterm::execute;
use crossterm::cursor::MoveTo;
use crossterm::terminal;
use strum::{EnumIter, IntoEnumIterator};
use unicode_segmentation::UnicodeSegmentation;
use crate::inductive::Formula;
use crate::proof::Proof;
use crate::rule::{Rule, RuleType, Side};
use crate::tools;
use deducnat_macro::{EnumType, EnumDoc};


const VERSION: &str = env!("CARGO_PKG_VERSION");

macro_rules! titleline {
    () => {
        println!("deducnat - v{VERSION}");
    };
}


#[derive(Clone)]
pub enum ReplState {
    Idle,
    Help(Box<ReplState>),
    CommandHelp(ReplCommand, Box<ReplState>),
    Proving(RefCell<Proof>, Vec<ReplCommand>),
    StepList(RefCell<Proof>, Vec<ReplCommand>),
    Qed(RefCell<Proof>, Vec<ReplCommand>),
    Quitting
}

impl PartialEq for ReplState {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (ReplState::Idle, ReplState::Idle) => true,
            (ReplState::Proving(_, _), ReplState::Proving(_, _)) => true,
            (ReplState::Quitting, ReplState::Quitting) => true,
            _ => false
        }
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}



#[derive(Clone, EnumIter, EnumDoc, EnumType)]
pub enum ReplCommand {
    #[cmd(name="proof", usage="<F>", desc="Start the proving process of F")]
    Proof(String),
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
    #[cmd(name="list", desc="Display the list of commands executed in proof mode")]
    List,
    #[cmd(name="qed", desc="Finish the proof (only when no more subgoals)")]
    Qed,

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
    ExFalso(String),

    // special command
    Return,             // Command when enter is pressed with no further input
}

impl Display for ReplCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReplCommand::Proof(s) => write!(f, "proof {s}"),
            ReplCommand::Trans(s) => write!(f, "trans {s}"),
            ReplCommand::AndLeft(s) => write!(f, "and_left {s}"),
            ReplCommand::AndRight(s) => write!(f, "and_right {s}"),
            ReplCommand::FromOr(s) => write!(f, "from_or {s}"),

            ReplCommand::Generalize(s) => write!(f, "gen {s}"),
            ReplCommand::FixAs(s) => write!(f, "fix_as {s}"),
            ReplCommand::Consider(s) => write!(f, "consider {s}"),
            ReplCommand::RenameAs(s) => write!(f, "rename_as {s}"),
            ReplCommand::ExFalso(s) => write!(f, "exfalso {s}"),
            ReplCommand::HelpCommand(s) => write!(f, "help {s}"),
            ReplCommand::Return => write!(f, ""),

            e => match e.name() {
                Some(n) => write!(f, "{n}"),
                None => Ok(()),
            }
        }
    }
}



#[derive(Debug)]
pub enum ReplError {
    TooMuchArguments,
    UnknownCommand(String),
    InvalidCommand,
    CommandError(String),
    UnableToRead
}

impl Display for ReplError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReplError::TooMuchArguments => write!(f, "too much arguments"),
            ReplError::UnknownCommand(s) => write!(f, "unknown command '{s}'"),
            ReplError::InvalidCommand => write!(f, "invalid command"),
            ReplError::CommandError(e) => write!(f, "{e}"),
            ReplError::UnableToRead => write!(f, "unable to read standard input"),
        }
    }
}



impl ReplCommand {
    pub fn from(command: &str) -> Result<ReplCommand, ReplError> {
        let command = command.trim();
        if command.is_empty() {return Ok(ReplCommand::Return);}

        let (cname, cparam) = match command.split_once(' ') {
            None => (command, ""),
            Some(c) => c
        };

        let cmd = match (cname, cparam) {
            ("proof", s) => ReplCommand::Proof(s.to_string()),

            ("axiom", "") => ReplCommand::Axiom,
            ("intro", "") => ReplCommand::Intro,
            ("intros", "") => ReplCommand::Intros,

            ("split", "") => ReplCommand::Split,

            ("trans", s) => ReplCommand::Trans(s.to_string()),

            ("and_left", s) => ReplCommand::AndLeft(s.to_string()),
            ("and_right", s) => ReplCommand::AndRight(s.to_string()),

            ("keep_left", "") => ReplCommand::KeepLeft,
            ("keep_right", "") => ReplCommand::KeepRight,

            ("from_or", s) => ReplCommand::FromOr(s.to_string()),

            ("gen", s) => ReplCommand::Generalize(s.to_string()),

            ("fix_as", s) => ReplCommand::FixAs(s.to_string()),
            ("consider", s) => ReplCommand::Consider(s.to_string()),
            ("rename_as", s) => ReplCommand::RenameAs(s.to_string()),

            ("from_bottom", "") => ReplCommand::FromBottom,
            ("exfalso", s) => ReplCommand::ExFalso(s.to_string()),


            ("qed", "") => ReplCommand::Qed,

            ("quit", _) => ReplCommand::Quit,
            ("exit", _) => ReplCommand::Exit,
            ("undo", _) => ReplCommand::Undo,
            ("help", "") => ReplCommand::Help,
            ("help", s) => ReplCommand::HelpCommand(s.to_string()),
            ("list", _) => ReplCommand::List,


            ("split", _) => return Err(ReplError::TooMuchArguments),
            ("keep_left", _) | ("keep_right", _) => return Err(ReplError::TooMuchArguments),
            ("intro", _) => return Err(ReplError::TooMuchArguments),
            ("axiom", _) => return Err(ReplError::TooMuchArguments),
            ("qed", _) => return Err(ReplError::TooMuchArguments),
            (c, _) => return Err(ReplError::UnknownCommand(c.to_string()))
        };

        Ok(cmd)
    }








    pub fn schema(&self) -> Option<(Vec<String>, String)> {
        let (ante, cons) = match self {
            ReplCommand::Axiom => (vec![""], "Γ, F ⊢ F"),
            ReplCommand::Intro => (vec!["Γ, F ⊢ G"], "Γ ⊢ F => G"),
            ReplCommand::Trans(_) => (vec!["Γ ⊢ F => G", "Γ ⊢ F"], "Γ ⊢ G"),
            ReplCommand::Split => (vec!["Γ ⊢ F", "Γ ⊢ G"], "Γ ⊢ F /\\ G"),
            ReplCommand::AndLeft(_) => (vec!["Γ ⊢ F /\\ G"], "Γ ⊢ G"),
            ReplCommand::AndRight(_) => (vec!["Γ ⊢ G /\\ F"], "Γ ⊢ G"),
            ReplCommand::KeepLeft => (vec!["Γ ⊢ F"], "Γ ⊢ F \\/ G"),
            ReplCommand::KeepRight => (vec!["Γ ⊢ G"], "Γ ⊢ F \\/ G"),
            ReplCommand::FromOr(_) => (vec!["Γ ⊢ F1 \\/ F2", "Γ, F1 ⊢ H", "Γ, F2 ⊢ H"], "Γ ⊢ H"),
            ReplCommand::Generalize(_) => (vec!["Γ ⊢ forall v, F"], "Γ ⊢ F[v -> T]"),
            ReplCommand::FixAs(_) => (vec!["Γ ⊢ F[v -> T]"], "Γ ⊢ exists v, F"),
            ReplCommand::Consider(_) => (vec!["Γ ⊢ exists v, F", "Γ, F ⊢ G"], "Γ ⊢ G"),
            ReplCommand::RenameAs(_) => (vec!["Γ ⊢ forall/exists v, F[x -> v]"], "Γ ⊢ forall/exists x, F"),
            ReplCommand::FromBottom => (vec!["Γ, ~F ⊢ falsum"], "Γ ⊢ F"),
            ReplCommand::ExFalso(_) => (vec!["Γ ⊢ F", "Γ ⊢ ~F"], "Γ ⊢ falsum"),

            _ => return None
        };
        let ante_str: Vec<_> = ante.iter().map(|s| s.to_string()).collect();
        Some((ante_str, cons.to_string()))
    }
}



impl ReplCommandType {
    // Return a list of command type based on the rule type it can generate
    pub fn from_rule(rule: &RuleType) -> Vec<ReplCommandType> {
        match rule {
            RuleType::Axiom => vec![ReplCommandType::Axiom],
            RuleType::Intro => vec![ReplCommandType::Intro],
            RuleType::Intros => vec![ReplCommandType::Intros],
            RuleType::Trans => vec![ReplCommandType::Trans],
            RuleType::SplitAnd => vec![ReplCommandType::Split],
            RuleType::And => vec![ReplCommandType::AndRight, ReplCommandType::AndLeft],
            RuleType::Keep => vec![ReplCommandType::KeepRight, ReplCommandType::KeepLeft],
            RuleType::FromOr => vec![ReplCommandType::FromOr],
            RuleType::Generalize => vec![ReplCommandType::Generalize],
            RuleType::FixAs => vec![ReplCommandType::FixAs],
            RuleType::Consider => vec![ReplCommandType::Consider],
            RuleType::RenameAs => vec![ReplCommandType::RenameAs],
            RuleType::FromBottom => vec![ReplCommandType::FromBottom],
            RuleType::ExFalso => vec![ReplCommandType::ExFalso],
        }
    }
}





pub struct Repl {
    pub state: ReplState,
    last_error: Option<ReplError>
}

impl Repl {
    pub fn new() -> Repl {
        Repl { state: ReplState::Idle, last_error: None }
    }

    pub fn from(formula: String) -> Result<Repl, ReplError> {
        let mut repl = Repl::new();
        repl.execute(ReplCommand::Proof(formula))?;

        return Ok(repl);
    }
    

    pub fn start(&mut self) -> io::Result<()> {
        execute!(io::stdout(), terminal::EnterAlternateScreen)?;

        // Run Repl
        while self.state != ReplState::Quitting {
            self.update()?;

            match self.get_command() {
                Ok(c) => {
                    match self.execute(c) {
                        Ok(_) => self.last_error = None,
                        Err(e) => self.last_error = Some(e),
                    }
                },
                Err(e) => self.last_error = Some(e)
            }
        }

        execute!(io::stdout(), terminal::LeaveAlternateScreen)
    }


    fn update(&mut self) -> io::Result<()> {
        execute!(io::stdout(), MoveTo(0, 0), terminal::Clear(terminal::ClearType::FromCursorDown))?;

        match &self.state {

            ReplState::Idle => {
                titleline!();
                println!("type 'proof P' to start to prove P");
                println!("     'help' for command information");
                println!("     'quit' to leave");
            }

            ReplState::Help(_) => {
                titleline!();

                println!("(F, G: Formula, T: Term, v: variable)");
                println!();

                println!("COMMANDS (more info with 'help [command]')");
                println!();



                let strings = ReplCommand::iter()
                    .filter_map(|cmd| {
                        let desc_fmt = |d| format!("-- {d}");
                        let desc = cmd.desc().map_or("".to_string(), desc_fmt);
                        if let Some(n) = cmd.name() {
                            Some((n, desc))
                        }
                        else {None}
                    })
                    .map(|(name, desc)| format!("{:10} {}", name, desc))
                    .collect::<Vec<String>>();

                let cols = tools::in_columns(&strings, terminal::size()?.0 as usize);
                println!("{cols}");
            }



            ReplState::CommandHelp(command, _) => {
                titleline!();
                println!("(F, G: Formula, T: Term, v: variable)");
                println!();

                if let Some(name) = command.name() {
                    println!("COMMAND '{}'", name.to_uppercase());
                }

                if let Some(s) = command.desc() {
                    println!("{s}");
                }
                println!();

                if let Some(usg) = command.usage() {
                    println!("USAGE: {usg}");
                }

                if let Some(schema) = command.schema() {
                    println!();
                    println!("SCHEMA:");
                    println!();

                    // sum length of elements in schema.0 and add a padding computed later
                    let mut length_top = schema.0.iter().map(|x| x.graphemes(true).count()).sum::<usize>();
                    length_top += (schema.0.len() - 1) * 5;

                    let length_bot = schema.1.graphemes(true).count();

                    let left_top_padding = if length_top < length_bot {(length_bot - length_top) / 2} else {0};
                    let left_bot_padding = if length_top > length_bot {(length_top - length_bot) / 2} else {0};

                    let antecedents_str = tools::list_str(&schema.0, " ".repeat(5).as_str());

                    println!("{}{}", " ".repeat(left_top_padding), antecedents_str);
                    println!("{}", "─".repeat(max(length_top, length_bot)));
                    println!("{}{}", " ".repeat(left_bot_padding), schema.1);
                }
            }



            ReplState::Proving(p, _) => {
                p.borrow().print();
            }



            ReplState::Qed(p, steps) => {
                let cmd_strs = steps.iter()
                    .enumerate()
                    .map(|(i, e)| format!("{i}. {e}"))
                    .collect::<Vec<String>>();

                titleline!();
                println!("PROOF OF  {}", p.borrow().goal);
                println!();
                println!("DEDUCTION STEPS:");
                println!();

                let cols = tools::in_columns(&cmd_strs, terminal::size()?.0 as usize);
                println!("{cols}");
            }



            ReplState::StepList(p, steps) => {
                let cmd_strs = steps.iter()
                    .enumerate()
                    .map(|(i, e)| format!("{i}. {e}"))
                    .collect::<Vec<String>>();

                if p.borrow().is_finished() {println!("Goal: {} (finished)", p.borrow().goal)}
                else {println!("Goal: {}", p.borrow().goal)}

                println!();

                println!("COMMANDS HISTORY");
                println!();

                let cols = tools::in_columns(&cmd_strs, terminal::size()?.0 as usize);
                println!("{cols}");
            }


            ReplState::Quitting => {}
        }

        let applicable_rules = if let ReplState::Proving(p, _) = &self.state {
            match p.borrow().get_applicable_rules() {
                None => None,
                Some(l) => {
                    let res = l.iter()
                        .map(|rt| ReplCommandType::from_rule(rt))
                        .flatten()
                        .map(|x| x.get_default().name().unwrap_or("".to_string()))
                        .collect();

                    Some(res)
                }
            }
        }
        else { None };

        // Error msg & command prompt
        let final_row = terminal::window_size()?.rows;

        if let Some(e) = &self.last_error {
            execute!(io::stdout(), MoveTo(0, final_row-2))?;
            print!("Error: {e}");
        }
        else if let Some(rules) = applicable_rules {
            execute!(io::stdout(), MoveTo(0, final_row-2))?;
            print!("Possible commands: {}", tools::list_str(&rules, ", "));
        }

        execute!(io::stdout(), MoveTo(0, final_row-1))?;

        print!("> ");

        io::stdout().flush()
    }


    fn get_command(&mut self) -> Result<ReplCommand, ReplError> {
        let mut txt = String::new();
        match io::stdin().read_line(&mut txt) {
            Ok(_) => {}
            Err(_) => return Err(ReplError::UnableToRead)
        };

        ReplCommand::from(&txt)
    }







    fn execute(&mut self, command: ReplCommand) -> Result<(), ReplError> {
        match (&mut self.state, &command) {

            // Start of proof
            (ReplState::Idle, ReplCommand::Proof(p)) => {
                let formula = match Formula::from_str(&p) {
                    Ok(f) => f,
                    Err(e) => return Err(ReplError::CommandError(e))
                };

                let proof = Proof::start(formula);
                self.state = ReplState::Proving(RefCell::new(proof), Vec::new());

                Ok(())
            },

            (ReplState::Proving(ref mut p, cs), subcommand) => {
                macro_rules! apply_rule {
                    ($rule:expr) => {
                        match p.borrow_mut().apply($rule) {
                            Ok(_) => {
                                cs.push(command.clone());
                                Ok(())
                            },
                            Err(e) => Err(ReplError::CommandError(e))
                        }
                    };
                }

                match subcommand {
                    ReplCommand::Axiom => apply_rule!(Rule::Axiom),
                    ReplCommand::Intro => apply_rule!(Rule::Intro),
                    ReplCommand::Intros => apply_rule!(Rule::Intros),
                    ReplCommand::Trans(s) => apply_rule!(Rule::Trans(s.to_string())),
                    ReplCommand::Split => apply_rule!(Rule::SplitAnd),
                    ReplCommand::AndLeft(s) => apply_rule!(Rule::And(Side::Left, s.to_string())),
                    ReplCommand::AndRight(s) => apply_rule!(Rule::And(Side::Right, s.to_string())),
                    ReplCommand::KeepLeft => apply_rule!(Rule::Keep(Side::Left)),
                    ReplCommand::KeepRight => apply_rule!(Rule::Keep(Side::Right)),
                    ReplCommand::FromOr(s) => apply_rule!(Rule::FromOr(s.to_string())),
                    ReplCommand::FromBottom => apply_rule!(Rule::FromBottom),
                    ReplCommand::ExFalso(s) => apply_rule!(Rule::ExFalso(s.to_string())),
                    ReplCommand::Generalize(s) => apply_rule!(Rule::Generalize(s.to_string())),
                    ReplCommand::FixAs(s) => apply_rule!(Rule::FixAs(s.to_string())),
                    ReplCommand::Consider(s) => apply_rule!(Rule::Consider(s.to_string())),
                    ReplCommand::RenameAs(s) => apply_rule!(Rule::RenameAs(s.to_string())),

                    ReplCommand::Qed => {
                        if p.borrow().is_finished() {
                            cs.push(command.clone());
                            self.state = ReplState::Qed(p.clone(), cs.clone());
                            Ok(())
                        } else {
                            Err(ReplError::CommandError("Proof not finished".to_string()))
                        }
                    }


                    ReplCommand::List => {
                        self.state = ReplState::StepList(p.clone(), cs.clone());
                        Ok(())
                    }


                    ReplCommand::Undo => {
                        let previous_state = p.borrow_mut().previous_state.clone();

                        match previous_state {
                            Some(ref ps) => {
                                let mut command_list = cs.clone();
                                command_list.pop();

                                self.state = ReplState::Proving(RefCell::new(*ps.clone()), command_list);
                                Ok(())
                            },
                            None => Err(ReplError::CommandError("No previous operation".to_string())),
                        }
                    },

                    ReplCommand::Exit => {
                        self.state = ReplState::Idle;
                        Ok(())
                    }

                    ReplCommand::Return => {
                        Ok(())
                    }

                    ReplCommand::Quit => {
                        self.state = ReplState::Quitting;
                        Ok(())
                    }

                    ReplCommand::Help => {
                        self.state = ReplState::Help(Box::new(self.state.clone()));
                        Ok(())
                    }

                    ReplCommand::HelpCommand(s) => {
                        let command = ReplCommand::from(s)?;

                        if let ReplState::CommandHelp(_, prev) = &self.state {
                            self.state = ReplState::CommandHelp(command, prev.clone());
                        }
                        else {
                            self.state = ReplState::CommandHelp(command, Box::new(self.state.clone()));
                        }

                        Ok(())
                    }

                    _ => Err(ReplError::InvalidCommand)
                }
            }






            (ReplState::Qed(_, _), ReplCommand::Exit | ReplCommand::Return) => {
                self.state = ReplState::Idle;
                Ok(())
            }



            (_, ReplCommand::Quit) => {
                self.state = ReplState::Quitting;
                Ok(())
            }


            (_, ReplCommand::Help) => {
                self.state = ReplState::Help(Box::new(self.state.clone()));
                Ok(())
            }

            (_, ReplCommand::HelpCommand(s)) => {
                let command = ReplCommand::from(s)?;

                if let ReplState::CommandHelp(_, prev) = &self.state {
                    self.state = ReplState::CommandHelp(command, prev.clone());
                }
                else {
                    self.state = ReplState::CommandHelp(command, Box::new(self.state.clone()));
                }

                Ok(())
            }

            
            (ReplState::Help(state) | ReplState::CommandHelp(_, state), ReplCommand::Exit | ReplCommand::Return) => {
                self.state = *state.to_owned();
                Ok(())
            }


            (ReplState::StepList(p, l), ReplCommand::Exit | ReplCommand::Return) => {
                self.state = ReplState::Proving(p.clone(), l.clone());
                Ok(())
            }



            // Do nothing
            (_, ReplCommand::Return) => Ok(()),



            _ => Err(ReplError::InvalidCommand)
        }
    }
}