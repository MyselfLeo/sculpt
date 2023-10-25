mod inductive;
mod parser;
mod sequent;


use crate::inductive::Formula;


fn main() {
    let formula_str = "((~A \\/ B) => (C => D) /\\ C)";
    let formula = Formula::from_str("((~A \\/ B) => (C => D) /\\ C)").unwrap();
    println!("{formula_str}");
    println!("{formula}");
}
