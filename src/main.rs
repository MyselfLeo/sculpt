mod inductive;
mod parser;
mod sequent;
mod rule;
mod proof;


use crate::{inductive::Formula, sequent::Sequent, rule::Intro};


fn main() {
    let formula = Formula::from_str("((~A \\/ B) => (C => D) /\\ C)").unwrap();
    let goal = Sequent::start(formula);
    println!("{goal}");

    let new_goals = goal.apply_rule(Box::new(Intro {hyp_name: "h1".to_string()})).unwrap();
    for s in new_goals {println!("{s}")}
}
