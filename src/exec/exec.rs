use std::fs;
use std::path::Path;
use crate::error::Error;
use crate::interpreter::{Interpreter, InterpreterCommand, InterpreterEffect};

const STEP_SEP: char = '.';

#[derive(Debug, Clone)]
pub struct Step {
    pub command_txt: String,
    pub start: (usize, usize), // line, column, inclusive
    pub end: (usize, usize),   // line, column, inclusive
}


/// Reads a file and interpret it either step by step or all at once.
pub struct Executor {
    pub filepath: String,

    pub steps: Vec<Step>,
    current_step: usize,

    interpreter: Interpreter
}

impl Executor {
    pub fn from_file(path: String) -> Result<Executor, Error> {
        let content = fs::read_to_string(&path).or_else(|_| Err(Error::UnableToRead))?;
        let steps = Self::parse_steps(&content)?;


        if steps.len() == 0 { return Err(Error::EmptyFile(path.clone())) }

        let filename = Path::new(&path)
            .file_name()
            .map_or("UNKNOWN".to_string(), |s| s.to_str().unwrap().to_string());

        Ok(Executor {filepath: path, steps, current_step: 0, interpreter: Interpreter::new(filename)})
    }


    pub fn current_step(&self) -> Step {
        self.steps[self.current_step].clone()
    }



    pub fn exec_all(&mut self) -> Result<(), (Error, Step)> {
        let steps = self.steps.clone();
        for s in steps {
            match self.exec_one(&s) {
                Ok(_) => (),
                Err(e) => return Err((e, s.clone()))
            }
        }

        if self.interpreter.current_proof.is_some() {
            return Err((Error::UnfinishedProof, self.steps.last().unwrap().clone()))
        }

        Ok(())
    }

    fn exec_one(&mut self, step: &Step) -> Result<(), Error> {
        let cmd = InterpreterCommand::from(&step.command_txt)?;
        match self.interpreter.execute(cmd)? {
            InterpreterEffect::NewTheorem(f) => println!("Added `{f}` to context"),
            _ => ()
        };

        Ok(())
    }



    fn filename(&self) -> String {
        let path = Path::new(&self.filepath);
        path.file_name().map_or("UNKNOWN".to_string(), |s| s.to_str().unwrap().to_string())
    }

    fn parse_steps(content: &String) -> Result<Vec<Step>, Error> {
        let mut res = vec![];

        let mut line_nb = 0;
        let mut column_nb = 0;

        let mut new_buf = false;
        let mut buf_start = (0, 0);
        let mut buf = String::new();

        for c in content.chars() {
            if c == '\r' {
                column_nb += 1;
                continue
            }
            if c == '\n' {
                line_nb += 1;
                column_nb = 0;
                continue;
            }
            if c == STEP_SEP {
                if buf.len() > 0 {
                    res.push(Step {
                        command_txt: buf.trim().to_string(),
                        start: buf_start,
                        end: (line_nb, column_nb-1)
                    })
                }
                new_buf = true;
                buf.clear();
                column_nb += 1;
                continue;
            }

            if new_buf {
                new_buf = false;
                buf_start = (line_nb, column_nb);
            }
            column_nb += 1;
            buf.push(c)
        }

        if !buf.is_empty() {
            return Err(Error::UnexpectedEOF)
        }

        Ok(res)
    }
}