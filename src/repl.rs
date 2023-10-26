use std::io;
use std::io::{Read, Write};
use crossterm::{execute, queue, QueueableCommand};
use crossterm::cursor::MoveTo;
use crossterm::terminal;
use crate::proof::Proof;

pub enum ReplState {
    Idle,
    Proving(Proof)
}


pub enum ReplCommand {
    Proof(String),
    Intro(String),
    Split(Option<String>),
    Elim(String),
    Axiom,
    Qed,

    Quit
}


pub enum ReplError {
    EmptyCommand,
    TooMuchArguments,
    UnknownCommand,
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
            ("Proof", s) => ReplCommand::Proof(s.to_string()),
            ("intro", s) => ReplCommand::Intro(s.to_string()),

            ("split", "") => ReplCommand::Split(None),
            ("split", s) => ReplCommand::Split(Some(s.to_string())),

            ("elim", s) => ReplCommand::Elim(s.to_string()),
            ("axiom", "") => ReplCommand::Axiom,

            ("Qed", "") => ReplCommand::Qed,

            ("Quit", "") => ReplCommand::Quit,

            ("axiom", s) => return Err(ReplError::TooMuchArguments),
            ("Qed", s) => return Err(ReplError::TooMuchArguments),
            ("Quit", s) => return Err(ReplError::TooMuchArguments),
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
        let quit = false;
        while !quit {
            self.print_screen()?;

            match self.get_command() {
                Ok(_) => {}
                Err(_) => {}
            }
        }

        execute!(io::stdout(), terminal::LeaveAlternateScreen)
    }


    fn print_screen(&mut self) -> io::Result<()> {
        queue!(io::stdout(), MoveTo(0, 0), terminal::Clear(terminal::ClearType::FromCursorDown));

        match self.state {
            ReplState::Idle => {
                println!("deducnat - v0.1.0");
                println!("type 'help' for command information");
            }
            ReplState::Proving(_) => {}
        }

        // Command prompt
        let final_row = terminal::window_size()?.rows - 1;
        queue!(io::stdout(), MoveTo(final_row, 0));
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
}