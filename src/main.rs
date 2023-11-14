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
    let token_str = tokens
        .iter()
        .map(|t| format!("{} ", t.to_string()))
        .collect::<String>();

    println!("{formula}");
    println!("\n");

    println!("{token_str}");
    println!("\n");
}
