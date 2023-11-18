mod inductive;
mod sequent;
mod rule;
mod proof;
mod repl;
mod tools;

use inductive::Term;
use lalrpop_util::lalrpop_mod;

use crate::repl::Repl;


lalrpop_mod!(pub formula);


fn start_repl() {
    let mut repl = Repl::new();
    repl.start().unwrap();
}


fn main() {
    //let formula = formula::FormulaParser::from_str("(forall x, H(x) => M(x)) => (H(Socrate)) => M(Socrate)");
    /*let tokens = parsing::lexer::lex(formula).unwrap();
    let token_str = tokens
        .iter()
        .map(|t| format!("{} ", t.to_string()))
        .collect::<String>();

    println!("{formula}");
    println!("\n");

    println!("{token_str}");
    println!("\n");*/


    let term = "f(x, f(y))";
    let nt: Box<Term> = formula::TermParser::new().parse(term).unwrap();

    println!("{:?}", nt);
}
