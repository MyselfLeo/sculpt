use crate::inductive::Formula;


pub struct Hypothesis {
    pub name: String,
    pub formula: Box<Formula>
}

pub struct Sequent {
    pub antecedents: Vec<Hypothesis>,
    pub goal: Formula
}


impl Sequent {
    // pub fn apply_rule(rule: Rule) -> Vec<Sequent> {
    // }
}