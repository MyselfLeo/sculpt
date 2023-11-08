mod inductive;
mod parser;
mod sequent;
mod rule;
mod proof;
mod repl;
mod tools;


use crate::repl::Repl;

fn main() {
    let mut repl = Repl::new();
    repl.start().unwrap();
}
