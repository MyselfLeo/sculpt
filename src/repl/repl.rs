use std::cmp::max;
use std::io;
use std::io::Write;
use crossterm::cursor::MoveTo;
use crossterm::{execute, terminal};
use unicode_segmentation::UnicodeSegmentation;
use crate::error::Error;
use crate::interpreter::Interpreter;
use crate::repl::command::{Command, ReplCommand, ReplCommandReplDoc};
use crate::tools;

const VERSION: &str = env!("CARGO_PKG_VERSION");

macro_rules! titleline {
    () => {
        println!("deducnat REPL - v{VERSION}");
    };
    ($s:expr) => {
        let upcs = $s.to_uppercase();
        println!("deducnat REPL - {upcs} - v{VERSION}");
    }
}



#[derive(Clone)]
pub enum ReplState {
    Idle,
    Working(Box<Interpreter>, Box<ReplState>),
    Help(Box<ReplState>),
    CommandHelp(Command, Box<ReplState>),
    Quitting
}

impl ReplState {
    pub fn is_quitting(&self) -> bool {
        match self {
            ReplState::Quitting => true,
            _ => false
        }
    }
}

/*impl PartialEq for ReplState {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (ReplState::Idle, ReplState::Idle) => true,
            //(ReplState::Proving(_, _, _), ReplState::Proving(_, _, _)) => true,
            (ReplState::Quitting, ReplState::Quitting) => true,
            _ => false
        }
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}*/






pub struct Repl {
    pub state: ReplState,
    last_error: Option<Error>
}

impl Repl {
    pub fn new() -> Repl {
        Repl { state: ReplState::Idle, last_error: None }
    }

    pub fn get_valid_commands(&self) -> Vec<Command> {
        let mut res = vec![Command::ReplCommand(ReplCommand::Quit)];
        match &self.state {
            ReplState::Idle => {
                res.push(Command::ReplCommand(ReplCommand::Context("".to_string())));
            }
            ReplState::Working(inter, _) => {
                res.extend(inter.get_valid_commands()
                   .iter()
                   .map(|cmd| Command::InterpreterCommand(cmd.clone()))
                   .collect::<Vec<_>>()
                );
                res.push(Command::ReplCommand(ReplCommand::Undo));
                res.push(Command::ReplCommand(ReplCommand::Exit));
                res.push(Command::ReplCommand(ReplCommand::Help));
            }
            ReplState::Help(_) => {
                res.push(Command::ReplCommand(ReplCommand::Exit));
            }
            ReplState::CommandHelp(_, _) => {
                res.push(Command::ReplCommand(ReplCommand::Exit));
            }
            ReplState::Quitting => ()
        };

        res.sort_by_key(|c| c.name());

        res
    }




    fn get_command(&mut self) -> Result<Command, Error> {
        let mut txt = String::new();
        match io::stdin().read_line(&mut txt) {
            Ok(_) => {}
            Err(_) => return Err(Error::UnableToRead)
        };

        Command::from(&txt)
    }





    pub fn start(&mut self) -> io::Result<()> {
        execute!(io::stdout(), terminal::EnterAlternateScreen)?;

        // Run Repl
        while !self.state.is_quitting() {
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

        match &mut self.state {

            ReplState::Idle => {
                titleline!();
                println!("type 'context <name>' to create a new context");
                println!("     'help' for command information");
                println!("     'quit' to leave");
            }

            ReplState::Help(_) => {
                titleline!();

                println!("(F, G: Formula, T: Term, v: variable)");
                println!();

                println!("COMMANDS (more info with 'help [command]')");
                println!();

                // todo: query every command

                /*let strings = Command::iter()
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
                println!("{cols}");*/
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


            ReplState::Working(ref mut ctx, _) => {
                titleline!(ctx.name);
                match &ctx.current_proof {
                    None => {
                        println!("type 'proof P' to start to prove P");
                        println!("     'admit F' to add an unproven assumption");
                        println!("     'help' for command information");
                        println!("     'quit' to leave");

                        println!();

                        println!("Context:");
                        let assumptions = ctx.context.iter()
                            .map(|f| f.to_string())
                            .collect::<Vec<_>>();

                        println!("{}", tools::in_columns(&assumptions, terminal::size()?.0 as usize));
                    }
                    Some(p) => p.print(),
                }
            }




            /*ReplState::Qed(p, steps) => {
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
            }*/



            /*ReplState::StepList(p, steps) => {
                titleline!(p.get_name());

                let cmd_strs = steps.iter()
                    .enumerate()
                    .map(|(i, e)| format!("{i}. {e}"))
                    .collect::<Vec<String>>();

                //if p.borrow().is_finished() {println!("Goal: {} (finished)", p.borrow().goal)}
                //else {println!("Goal: {}", p.borrow().goal)}

                println!();

                println!("COMMANDS HISTORY");
                println!();

                let cols = tools::in_columns(&cmd_strs, terminal::size()?.0 as usize);
                println!("{cols}");
            }*/


            ReplState::Quitting => {}
        }

        // Error msg & command prompt
        let final_row = terminal::window_size()?.rows;

        let valid_command_str = self.get_valid_commands().iter()
            .map(|cmd| cmd.name().unwrap_or(String::new()))
            .collect::<Vec<_>>();

        if let Some(e) = &self.last_error {
            execute!(io::stdout(), MoveTo(0, final_row-2))?;
            print!("Error: {e}");
        }
        else if !valid_command_str.is_empty() {
            execute!(io::stdout(), MoveTo(0, final_row-2))?;
            print!("Possible commands: {}", tools::list_str(&valid_command_str, ", "));
        }

        execute!(io::stdout(), MoveTo(0, final_row-1))?;

        print!("> ");

        io::stdout().flush()
    }








    pub fn execute(&mut self, cmd: Command) -> Result<(), Error> {

        let previous = |s: &ReplState| {
            if let ReplState::Help(prev) = s {
                prev.clone()
            } else if let ReplState::CommandHelp(_, prev) = s {
                prev.clone()
            } else {
                Box::new(s.clone())
            }
        };

        let curr_clone = self.state.clone();

        match (&mut self.state, cmd) {

            // Terminating
            (_, Command::ReplCommand(ReplCommand::Quit)) => {
                self.state = ReplState::Quitting;
            }


            // Treat repl-exclusive commands first
            (state, Command::ReplCommand(c)) => {
                match (state, c) {
                    // Start context
                    (ReplState::Idle, ReplCommand::Context(s)) => {
                        let ctx = Interpreter::new(s);
                        self.state = ReplState::Working(Box::new(ctx), Box::new(ReplState::Idle));
                    }

                    // Revert operation
                    (ReplState::Working(_, prev), ReplCommand::Undo) => {
                        self.state = *prev.clone();
                    },

                    // Go to help page
                    (s, ReplCommand::Help) => {
                        // if the previous state is also Help or CommandHelp, we use this state's
                        // previous instead of itself to prevent huge help-screen history

                        self.state = ReplState::Help(previous(s));
                    },
                    (s, ReplCommand::HelpCommand(cmd)) => {
                        let command = Command::from(&cmd)?;

                        // if the previous state is also Help or CommandHelp, we use this state's
                        // previous instead of itself to prevent huge help-screen history
                        self.state = ReplState::CommandHelp(command, previous(s));
                    },

                    // Exit help menu
                    (ReplState::Help(s), ReplCommand::Exit | ReplCommand::Return) => {
                        self.state = *s.clone();
                    }
                    (ReplState::CommandHelp(_, s), ReplCommand::Exit | ReplCommand::Return) => {
                        self.state = *s.clone();
                    },

                    // 'Return' only has an effect on sub-screens (Help, CommandHelp))
                    (_, ReplCommand::Return) => (),

                    (_, c) => {
                        return Err(Error::InvalidCommand(c.name().unwrap_or("unknown".to_string())))
                    }
                }
            }



            // Other commands (that could be found in a file for example)
            (ReplState::Working(ref mut inter, ref mut prev), Command::InterpreterCommand(cmd)) => {
                inter.execute(cmd)?;
                *prev = Box::new(curr_clone);
            },


            (_, cmd) => {
                return Err(Error::InvalidCommand(cmd.name().unwrap_or("unknown".to_string())))
            }


        };

        Ok(())
    }
}