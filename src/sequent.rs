use crate::inductive::Formula;
use crate::rule::Rule;

 
pub struct Hypothesis {
    pub name: String,
    pub formula: Box<Formula>
}

pub struct Sequent {
    pub antecedents: Vec<Hypothesis>,
    pub goal: Box<Formula>
}


impl Sequent {
    pub fn apply_rule(&self, rule: Box<dyn Rule>) -> Result<Vec<Sequent>, ()> {
        rule.apply(self)
    }
}