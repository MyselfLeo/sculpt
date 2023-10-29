use std::cell::RefCell;
use std::fmt::Display;
use std::io;
use std::io::Write;
use crossterm::execute;
use crossterm::cursor::MoveTo;
use crossterm::terminal;
use crate::inductive::Formula;
use crate::proof::Proof;
use crate::rule::{Rule, Side};


#[derive(Clone)]
pub enum ReplState {
    Idle,
    Help(Box<ReplState>),
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

#[derive(Clone)]
pub enum ReplCommand {
    Proof(String),
    Axiom,
    Intro,
    Trans(String),
    Split,
    AndLeft(String),
    AndRight(String),
    KeepLeft,
    KeepRight,
    FromOr(String),
    Qed,
    List,
    Undo,

    Quit,
    Exit,
    Help,
    Return          // Command when enter is pressed with no further input
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
            ReplCommand::Qed => write!(f, "qed"),
            ReplCommand::Quit => write!(f, "quit"),
            ReplCommand::Exit => write!(f, "exit"),
            ReplCommand::Help => write!(f, "help"),
            ReplCommand::List => write!(f, "list"),
            ReplCommand::Undo => write!(f, "undo"),
            ReplCommand::Return => write!(f, "[return]")
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


            ("qed", "") => ReplCommand::Qed,

            ("quit", _) => ReplCommand::Quit,
            ("exit", _) => ReplCommand::Exit,
            ("undo", _) => ReplCommand::Undo,
            ("help", _) => ReplCommand::Help,
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
}



pub struct Repl {
    pub state: ReplState,
    last_error: Option<ReplError>
}

impl Repl {
    pub fn new() -> Repl {
        Repl { state: ReplState::Idle, last_error: None }
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
                println!("deducnat - v0.1.0");
                println!("type 'proof P' to start to prove P");
                println!("     'help' for command information");
                println!("     'quit' to leave");
            }

            ReplState::Help(_) => {
                println!("deducnat - v0.1.0");
                println!();

                println!("MAIN COMMANDS");
                println!("help                    -- Display this information screen");
                println!("exit                    -- Close sub-screens (help, list) or go back to main screen");
                println!("quit                    -- Stop deducnat");
                println!("proof <prop>            -- Start the proving process for prop");
                println!();

                println!("PROOF COMMANDS (P, Q: propositions)");
                println!("qed                     -- Finish the proof (only when no more subgoals)");
                println!("list                    -- Display the list of commands executed for this proof");
                println!("undo                    -- Revert last operation");
                println!("");

                println!("axiom");
                println!("intro");
                println!("split");
                println!("trans <P>");
                println!("and_left <P>");
                println!("and_right <P>");
                println!("keep_left");
                println!("keep_right");
                println!("from_or <P \\/ Q>");
            }



            ReplState::Proving(p, _) => {
                p.borrow().print();
            }



            ReplState::Qed(p, steps) => {
                println!("PROOF OF  {}", p.borrow().goal);
                println!();
                println!("DEDUCTION STEPS:");
                for s in steps {
                    println!("{s}");
                }
            }



            ReplState::StepList(p, steps) => {
                if p.borrow().is_finished() {println!("Goal: {} (finished)", p.borrow().goal)}
                else {println!("Goal: {}", p.borrow().goal)}

                println!();

                println!("COMMANDS");
                for s in steps {
                    println!("{s}");
                }
            }


            ReplState::Quitting => {}
        }

        // Error msg & command prompte
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


            (ReplState::Proving(ref mut p, cs), ReplCommand::Axiom) => {
                match &p.borrow_mut().apply(Rule::Axiom) {
                    Ok(_) => {
                        cs.push(command.clone());
                        Ok(())
                    },
                    Err(e) => Err(ReplError::CommandError(e.clone()))
                }
            },


            (ReplState::Proving(ref mut p, cs), ReplCommand::Intro) => {
                match p.borrow_mut().apply(Rule::Intro) {
                    Ok(_) => {
                        cs.push(command.clone());
                        Ok(())
                    },
                    Err(e) => Err(ReplError::CommandError(e))
                }
            },


            (ReplState::Proving(ref mut p, cs), ReplCommand::Trans(s)) => {
                match p.borrow_mut().apply(Rule::Trans(s.to_string())) {
                    Ok(_) => {
                        cs.push(command.clone());
                        Ok(())
                    },
                    Err(e) => Err(ReplError::CommandError(e))
                }
            },


            (ReplState::Proving(ref mut p, cs), ReplCommand::Split) => {
                match p.borrow_mut().apply(Rule::SplitAnd) {
                    Ok(_) => {
                        cs.push(command.clone());
                        Ok(())
                    },
                    Err(e) => Err(ReplError::CommandError(e))
                }
            },


            (ReplState::Proving(ref mut p, cs), ReplCommand::AndLeft(s)) => {
                match p.borrow_mut().apply(Rule::And(Side::Left, s.to_string())) {
                    Ok(_) => {
                        cs.push(command.clone());
                        Ok(())
                    },
                    Err(e) => Err(ReplError::CommandError(e))
                }
            },


            (ReplState::Proving(ref mut p, cs), ReplCommand::AndRight(s)) => {
                match p.borrow_mut().apply(Rule::And(Side::Right, s.to_string())) {
                    Ok(_) => {
                        cs.push(command.clone());
                        Ok(())
                    },
                    Err(e) => Err(ReplError::CommandError(e))
                }
            },


            (ReplState::Proving(ref mut p, cs), ReplCommand::KeepLeft) => {
                match p.borrow_mut().apply(Rule::Keep(Side::Left)) {
                    Ok(_) => {
                        cs.push(command.clone());
                        Ok(())
                    },
                    Err(e) => Err(ReplError::CommandError(e))
                }
            },


            (ReplState::Proving(ref mut p, cs), ReplCommand::KeepRight) => {
                match p.borrow_mut().apply(Rule::Keep(Side::Right)) {
                    Ok(_) => {
                        cs.push(command.clone());
                        Ok(())
                    },
                    Err(e) => Err(ReplError::CommandError(e))
                }
            },


            (ReplState::Proving(ref mut p, cs), ReplCommand::FromOr(s)) => {
                match p.borrow_mut().apply(Rule::FromOr(s.to_string())) {
                    Ok(_) => {
                        cs.push(command.clone());
                        Ok(())
                    },
                    Err(e) => Err(ReplError::CommandError(e))
                }
            },


            (ReplState::Proving(ref mut p, cs), ReplCommand::Qed) => {
                if p.borrow().is_finished() {
                    cs.push(command.clone());
                    self.state = ReplState::Qed(p.clone(), cs.clone());
                    Ok(())
                }
                else {
                    Err(ReplError::CommandError("Proof not finished".to_string()))
                }
            }


            (ReplState::Qed(_, _) | ReplState::Proving(_, _), ReplCommand::Exit | ReplCommand::Return) => {
                self.state = ReplState::Idle;
                Ok(())
            }


            (ReplState::Proving(ref mut p, list), ReplCommand::List) => {
                self.state = ReplState::StepList(p.clone(), list.clone());
                Ok(())
            }


            (ReplState::Proving(ref mut p, list), ReplCommand::Undo) => {
                let previous_state = p.borrow_mut().previous_state.clone();
                
                match previous_state {
                    Some(ref ps) => {
                        let mut command_list = list.clone();
                        command_list.pop();

                        self.state = ReplState::Proving(RefCell::new(*ps.clone()), command_list);
                        Ok(())
                    },
                    None => Err(ReplError::CommandError("No previous operation".to_string())),
                }
            }


            (_, ReplCommand::Quit) => {
                self.state = ReplState::Quitting;
                Ok(())
            }


            (_, ReplCommand::Help) => {
                self.state = ReplState::Help(Box::new(self.state.clone()));
                Ok(())
            }

            
            (ReplState::Help(state), ReplCommand::Exit | ReplCommand::Return) => {
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