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
    let tokens = parser::lex(formula).unwrap();
    println!("{:?}", tokens);

    println!("\n\n");

    let postfix = parser::infix_to_postfix(&tokens).unwrap();
    println!("{:?}", tokens);

}
