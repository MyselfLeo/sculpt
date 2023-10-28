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
    Proving(Proof, Vec<ReplCommand>),
    CommandList(Proof, Vec<ReplCommand>),
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

    Quit,
    Exit,
    Help
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
            ReplCommand::List => write!(f, "list")
        }
    }
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
            ("help", _) => ReplCommand::Help,
            ("list", _) => ReplCommand::List,

            ("split", _) => return Err(ReplError::TooMuchArguments),
            ("keep_left", _) | ("keep_right", _) => return Err(ReplError::TooMuchArguments),
            ("intro", _) => return Err(ReplError::TooMuchArguments),
            ("axiom", _) => return Err(ReplError::TooMuchArguments),
            ("qed", _) => return Err(ReplError::TooMuchArguments),
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
        execute!(io::stdout(), MoveTo(0, 0), terminal::Clear(terminal::ClearType::FromCursorDown))?;

        match &self.state {

            ReplState::Idle => {
                println!("deducnat - v0.1.0");
                println!("type 'help' for command information, 'quit' to leave");
            }

            ReplState::Help(_) => {
                println!("deducnat - v0.1.0");
                println!();

                println!("MAIN COMMANDS");
                println!("help                    -- Display this information screen");
                println!("exit                    -- Close sub-screens (like help or list)");
                println!("quit                    -- Stop deducnat");
                println!("proof <prop>            -- Start the proving process for prop");
                println!();

                println!("PROOF COMMANDS");
                println!("qed                     -- Quit the proof only if finished");
                println!("list                    -- Display the list of commands executed for this proof");
                println!("");

                println!("axiom");
                println!("intro");
                println!("split");
                println!("trans <prop>");
                println!("and_left prop>");
                println!("and_right <prop>");
                println!("keep_left");
                println!("keep_right");
                println!("from_or <'or' prop>");
            }


            ReplState::Proving(p, _) => {
                p.print();
            }


            ReplState::CommandList(p, commands) => {
                match p.get_current_goal() {
                    None => println!("Goal: {} (finished)", p.goal),
                    Some(_) => println!("Goal: {}", p.goal)
                };

                println!();

                println!("COMMANDS");
                for c in commands {
                    println!("{c}");
                }
            }


            ReplState::Quitting => {}
        }

        // Command prompt
        let final_row = terminal::window_size()?.rows;
        execute!(io::stdout(), MoveTo(0, final_row))?;

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
                self.state = ReplState::Proving(proof, Vec::new());

                Ok(())
            },


            (ReplState::Proving(ref mut p, cs), ReplCommand::Axiom) => {
                match p.apply(Rule::Axiom) {
                    Ok(_) => {
                        cs.push(command.clone());
                        Ok(())
                    },
                    Err(e) => Err(ReplError::CommandError(e))
                }
            },


            (ReplState::Proving(ref mut p, cs), ReplCommand::Intro) => {
                match p.apply(Rule::Intro) {
                    Ok(_) => {
                        cs.push(command.clone());
                        Ok(())
                    },
                    Err(e) => Err(ReplError::CommandError(e))
                }
            },


            (ReplState::Proving(ref mut p, cs), ReplCommand::Trans(s)) => {
                match p.apply(Rule::Trans(s.to_string())) {
                    Ok(_) => {
                        cs.push(command.clone());
                        Ok(())
                    },
                    Err(e) => Err(ReplError::CommandError(e))
                }
            },


            (ReplState::Proving(ref mut p, cs), ReplCommand::Split) => {
                match p.apply(Rule::SplitAnd) {
                    Ok(_) => {
                        cs.push(command.clone());
                        Ok(())
                    },
                    Err(e) => Err(ReplError::CommandError(e))
                }
            },


            (ReplState::Proving(ref mut p, cs), ReplCommand::AndLeft(s)) => {
                match p.apply(Rule::And(Side::Left, s.to_string())) {
                    Ok(_) => {
                        cs.push(command.clone());
                        Ok(())
                    },
                    Err(e) => Err(ReplError::CommandError(e))
                }
            },


            (ReplState::Proving(ref mut p, cs), ReplCommand::AndRight(s)) => {
                match p.apply(Rule::And(Side::Right, s.to_string())) {
                    Ok(_) => {
                        cs.push(command.clone());
                        Ok(())
                    },
                    Err(e) => Err(ReplError::CommandError(e))
                }
            },


            (ReplState::Proving(ref mut p, cs), ReplCommand::KeepLeft) => {
                match p.apply(Rule::Keep(Side::Left)) {
                    Ok(_) => {
                        cs.push(command.clone());
                        Ok(())
                    },
                    Err(e) => Err(ReplError::CommandError(e))
                }
            },


            (ReplState::Proving(ref mut p, cs), ReplCommand::KeepRight) => {
                match p.apply(Rule::Keep(Side::Right)) {
                    Ok(_) => {
                        cs.push(command.clone());
                        Ok(())
                    },
                    Err(e) => Err(ReplError::CommandError(e))
                }
            },


            (ReplState::Proving(ref mut p, cs), ReplCommand::FromOr(s)) => {
                match p.apply(Rule::FromOr(s.to_string())) {
                    Ok(_) => {
                        cs.push(command.clone());
                        Ok(())
                    },
                    Err(e) => Err(ReplError::CommandError(e))
                }
            },


            (ReplState::Proving(ref mut p, _), ReplCommand::Qed) => {
                if p.is_finished() {
                    self.state = ReplState::Idle;
                    Ok(())
                }
                else {
                    Err(ReplError::CommandError("Proof not finished".to_string()))
                }
            }


            (ReplState::Proving(ref mut p, list), ReplCommand::List) => {
                self.state = ReplState::CommandList(p.clone(), list.clone());
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

            
            (ReplState::Help(state), ReplCommand::Exit) => {
                self.state = *state.to_owned();
                Ok(())
            }


            (ReplState::CommandList(p, l), ReplCommand::Exit) => {
                self.state = ReplState::Proving(p.clone(), l.clone());
                Ok(())
            }


            _ => Err(ReplError::UnknownCommand)
        }
    }
}