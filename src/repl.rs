use std::io;
use std::io::Write;
use crossterm::{execute, QueueableCommand};
use crossterm::cursor::MoveTo;
use crossterm::terminal;
use crate::inductive::Formula;
use crate::proof::Proof;
use crate::rule::Rule;


pub enum ReplState {
    Idle,
    Proving(Proof),
    Quitting
}

impl PartialEq for ReplState {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (ReplState::Idle, ReplState::Idle) => true,
            (ReplState::Proving(_), ReplState::Proving(_)) => true,
            (ReplState::Quitting, ReplState::Quitting) => true,
            _ => false
        }
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}


pub enum ReplCommand {
    Proof(String),
    Intro,
    Split(Option<String>),
    Elim(String),
    Axiom,
    Qed,

    Quit,
    Help
}


#[derive(Debug)]
pub enum ReplError {
    EmptyCommand,
    TooMuchArguments,
    UnknownCommand,
    InvalidCommand,
    CommandError(String),
    UnableToRead
}


impl ReplCommand {
    pub fn from(command: &str) -> Result<ReplCommand, ReplError> {
        let command = command.trim();
        if command.is_empty() {return Err(ReplError::EmptyCommand);}

        let (cname, cparam) = match command.split_once(' ') {
            None => (command, ""),
            Some(c) => c
        };

        let cmd = match (cname, cparam) {
            ("proof", s) => ReplCommand::Proof(s.to_string()),
            ("intro", "") => ReplCommand::Intro,

            ("split", "") => ReplCommand::Split(None),
            ("split", s) => ReplCommand::Split(Some(s.to_string())),

            ("elim", s) => ReplCommand::Elim(s.to_string()),
            ("axiom", "") => ReplCommand::Axiom,
            ("qed", "") => ReplCommand::Qed,

            ("quit", _) | ("exit", _) => ReplCommand::Quit,
            ("help", "") => ReplCommand::Help,

            ("intro", _) => return Err(ReplError::TooMuchArguments),
            ("axiom", _) => return Err(ReplError::TooMuchArguments),
            ("qed", _) => return Err(ReplError::TooMuchArguments),
            ("help", _) => return Err(ReplError::TooMuchArguments),
            _ => return Err(ReplError::UnknownCommand)
        };

        Ok(cmd)
    }
}



pub struct Repl {
    pub state: ReplState
}

impl Repl {
    pub fn new() -> Repl {
        Repl { state: ReplState::Idle }
    }

    pub fn start(&mut self) -> io::Result<()> {
        execute!(io::stdout(), terminal::EnterAlternateScreen)?;

        // Run Repl
        while self.state != ReplState::Quitting {
            self.update()?;

            match self.get_command() {
                Ok(c) => { self.execute(c).unwrap(); },
                Err(_) => {}
            }
        }

        execute!(io::stdout(), terminal::LeaveAlternateScreen)
    }


    fn update(&mut self) -> io::Result<()> {
        execute!(io::stdout(), MoveTo(0, 0), terminal::Clear(terminal::ClearType::FromCursorDown));

        match &self.state {
            ReplState::Idle => {
                println!("deducnat - v0.1.0");
                println!("type 'help' for command information, 'quit' to leave");
            }

            ReplState::Proving(p) => {
                p.print();
            }
            ReplState::Quitting => {}
        }

        // Command prompt
        let final_row = terminal::window_size()?.rows;
        execute!(io::stdout(), MoveTo(0, final_row));

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
        match command {
            ReplCommand::Proof(p) => {
                if self.state != ReplState::Idle {Err(ReplError::InvalidCommand)}
                else {
                    let formula = match Formula::from_str(&p) {
                        Ok(f) => f,
                        Err(e) => return Err(ReplError::CommandError(e))
                    };

                    let proof = Proof::start(formula);
                    self.state = ReplState::Proving(proof);

                    Ok(())
                }
            },
            ReplCommand::Intro => {
                match self.state {
                    ReplState::Proving(ref mut p) => match p.apply(Rule::Intro) {
                        Ok(_) => Ok(()),
                        Err(e) => Err(ReplError::CommandError(e))
                    },
                    _ => Err(ReplError::InvalidCommand)
                }
            },
            ReplCommand::Split(_) => unimplemented!(),
            ReplCommand::Elim(s) => {
                match self.state {
                    ReplState::Proving(ref mut p) => match p.apply(Rule::Elim(s)) {
                        Ok(_) => Ok(()),
                        Err(e) => Err(ReplError::CommandError(e))
                    },
                    _ => Err(ReplError::InvalidCommand)
                }
            },
            ReplCommand::Axiom => {
                match self.state {
                    ReplState::Proving(ref mut p) => match p.apply(Rule::Axiom) {
                        Ok(_) => Ok(()),
                        Err(e) => Err(ReplError::CommandError(e))
                    },
                    _ => Err(ReplError::InvalidCommand)
                }
            },
            ReplCommand::Qed => unimplemented!(),
            ReplCommand::Quit => {
                self.state = ReplState::Quitting;
                Ok(())
            },
            ReplCommand::Help => unimplemented!()
        }
    }
}