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
use crate::rule::{Rule, Side};
use crate::tools;
use deducnat_macro::ReplDoc;


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


pub trait ReplDoc {
    fn name(&self) -> String;
}


#[derive(Clone, EnumIter, ReplDoc)]
pub enum ReplCommand {
    #[cmd(name = "Proof")]
    Proof(String),
    Help,
    HelpCommand(String),
    Undo,
    Exit,
    Quit,
    List,
    Qed,

    Axiom,
    Intro,
    Trans(String),
    Split,
    AndLeft(String),
    AndRight(String),
    KeepLeft,
    KeepRight,
    FromOr(String),
    Generalize(String),
    FixAs(String),
    Consider(String),
    RenameAs(String),
    FromBottom,
    ExFalso(String),

    // special command
    Return,             // Command when enter is pressed with no further input
}

impl Display for ReplCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReplCommand::Proof(s) => write!(f, "proof {s}"),
            ReplCommand::Axiom => write!(f, "axiom"),
            ReplCommand::Intro => write!(f, "intro"),
            ReplCommand::Trans(s) => write!(f, "trans {s}"),
            ReplCommand::Split => write!(f, "split"),
            ReplCommand::AndLeft(s) => write!(f, "and_left {s}"),
            ReplCommand::AndRight(s) => write!(f, "and_right {s}"),
            ReplCommand::KeepLeft => write!(f, "keep_left"),
            ReplCommand::KeepRight => write!(f, "keep_right"),
            ReplCommand::FromOr(s) => write!(f, "from_or {s}"),

            ReplCommand::Generalize(s) => write!(f, "gen {s}"),
            ReplCommand::FixAs(s) => write!(f, "fix_as {s}"),
            ReplCommand::Consider(s) => write!(f, "consider {s}"),
            ReplCommand::RenameAs(s) => write!(f, "rename_as {s}"),

            ReplCommand::FromBottom => write!(f, "from_bottom"),
            ReplCommand::ExFalso(s) => write!(f, "exfalso {s}"),

            ReplCommand::Qed => write!(f, "qed"),
            ReplCommand::Quit => write!(f, "quit"),
            ReplCommand::Exit => write!(f, "exit"),
            ReplCommand::Help => write!(f, "help"),
            ReplCommand::HelpCommand(s) => write!(f, "help {s}"),
            ReplCommand::List => write!(f, "list"),
            ReplCommand::Undo => write!(f, "undo"),
            ReplCommand::Return => write!(f, "")
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


    pub fn name(&self) -> String {
        match self {
            ReplCommand::Proof(_) => "proof",
            ReplCommand::Help | ReplCommand::HelpCommand(_) => "help",
            ReplCommand::Undo => "undo",
            ReplCommand::Exit => "exit",
            ReplCommand::Quit => "quit",
            ReplCommand::List => "list",

            ReplCommand::Axiom => "axiom",
            ReplCommand::Intro => "intro",
            ReplCommand::Trans(_) => "trans",
            ReplCommand::Split => "split",
            ReplCommand::AndLeft(_) => "and_left",
            ReplCommand::AndRight(_) => "and_right",
            ReplCommand::KeepLeft => "keep_left",
            ReplCommand::KeepRight => "keep_right",
            ReplCommand::FromOr(_) => "from_or",
            ReplCommand::Generalize(_) => "gen",
            ReplCommand::FixAs(_) => "fix_as",
            ReplCommand::Consider(_) => "consider",
            ReplCommand::RenameAs(_) => "rename_as",
            ReplCommand::FromBottom => "from_bottom",
            ReplCommand::ExFalso(_) => "exfalso",

            ReplCommand::Qed => "qed",

            ReplCommand::Return => "",
        }.to_string()
    }
    */


    pub fn usage(&self) -> String {
        let args = match self {
            ReplCommand::Proof(_) => "<F>",
            ReplCommand::Trans(_) => "<F>",
            ReplCommand::AndLeft(_) => "<F>",
            ReplCommand::AndRight(_) => "<F>",

            ReplCommand::FromOr(_) => "<F1> \\/ <F2>",
            ReplCommand::Generalize(_) => "<T>",
            ReplCommand::FixAs(_) => "<T>",
            ReplCommand::Consider(_) => "exists <v>, <F>",
            ReplCommand::RenameAs(_) => "<v>",
            ReplCommand::ExFalso(_) => "<F>",
            ReplCommand::HelpCommand(_) => "[command]",
            _ => ""
        }.to_string();

        format!("{} {}", self.name(), args)
    }



    pub fn desc(&self) -> Option<String> {
        let res = match self {
            ReplCommand::Proof(_) => "Start the proving process of F",
            ReplCommand::Qed => "Finish the proof (only when no more subgoals)",
            ReplCommand::List => "Display the list of commands executed in proof mode",
            ReplCommand::Undo => "Revert last command while in proof mode",
            ReplCommand::Quit => "Stop deducnat",
            ReplCommand::Exit => "Close sub-screens (help, list) or go back to main screen",
            ReplCommand::Help => "Display this information screen",
            ReplCommand::HelpCommand(_) => "Display information about a particular command",
            _ => return None
        }.to_string();

        Some(res)
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
                    .map(|cmd| (format!("{:10}", cmd.name()), cmd.desc()))
                    .map(|(str, desc)| (str, desc.map_or("".to_string(), |d| format!("-- {d}"))))
                    .map(|(str, desc)| format!("{str} {desc}"))
                    .collect::<Vec<String>>();

                let cols = tools::in_columns(&strings, terminal::size()?.0 as usize);
                println!("{cols}");
            }



            ReplState::CommandHelp(command, _) => {
                titleline!();
                println!("(F, G: Formula, T: Term, v: variable)");
                println!();

                println!("COMMAND '{}'", command.name().to_uppercase());
                if let Some(s) = command.desc() {
                    println!("{s}");
                }
                println!();
                println!("USAGE: {}", command.usage());

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

        // Error msg & command prompt
        let final_row = terminal::window_size()?.rows;

        if let Some(e) = &self.last_error {
            execute!(io::stdout(), MoveTo(0, final_row-2))?;
            print!("Error: {e}");
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