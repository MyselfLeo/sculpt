use crate::inductive::Formula;


pub struct Hypothesis {
    pub name: String,
    pub formula: Box<Formula>
}

pub struct Sequent {
    pub antecedents: Vec<Hypothesis>,
    pub consequent: Formula
}