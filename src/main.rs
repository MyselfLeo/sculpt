mod inductive;
mod sequent;
mod rule;
mod proof;
mod repl;
mod tools;


use lalrpop_util::lalrpop_mod;


use inductive::Formula;
use repl::Repl;


lalrpop_mod!(pub parser);



fn start_repl() {
    let mut repl = Repl::new();
    repl.start().unwrap();
}


fn main() {
    start_repl();
}
