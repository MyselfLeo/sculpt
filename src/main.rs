mod inductive;
mod parser;
mod sequent;
mod rule;
mod proof;
mod repl;
mod tools;

use crate::repl::Repl;


fn start_repl() {
    let mut repl = Repl::new();
    repl.start().unwrap();
}


fn main() {
    let formula = "(forall x, H(x) => M(x)) => (H(Socrate)) => M(Socrate)";
    println!("{:#?}", parser::lex(formula));

}
