mod inductive;
mod parser;
mod sequent;
mod rule;
mod proof;


use crate::inductive::Formula;
use crate::proof::Proof;
use crate::rule::Rule;

// "((~A \\/ B) => (C => D) /\\ C)"

// (A => B) => A => B

fn main() {
    let formula = Formula::from_str("(A => B) => A => B").unwrap();
    let mut proof = Proof::start(formula);
    proof.print();

    proof.apply(Rule::Intro("h1".to_string())).unwrap();
    println!("\n\n");
    proof.print();

    proof.apply(Rule::Intro("h2".to_string())).unwrap();
    println!("\n\n");
    proof.print();

    proof.apply(Rule::Elim("A".to_string())).unwrap();
    println!("\n\n");
    proof.print();

    proof.apply(Rule::Axiom).unwrap();
    println!("\n\n");
    proof.print();

    proof.apply(Rule::Axiom).unwrap();
    println!("\n\n");
    proof.print();
}
