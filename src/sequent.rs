use std::fmt::Display;

use crate::inductive::Formula;
use crate::rule::Rule;

 
 #[derive(Clone)]
pub struct Hypothesis {
    pub name: String,
    pub formula: Box<Formula>
}

pub struct Sequent {
    pub antecedents: Vec<Hypothesis>,
    pub goal: Box<Formula>
}


impl Sequent {
    pub fn start(formula: Box<Formula>) -> Sequent {
        Sequent { antecedents: vec![], goal: formula }
    }

    pub fn apply_rule(&self, rule: Box<dyn Rule>) -> Result<Vec<Sequent>, ()> {
        rule.apply(self)
    }
}

impl Display for Sequent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for h in &self.antecedents {
            write!(f, "{}: {}\n", h.name, h.formula)?;
        }
        write!(f, "──────────────────────────\n")?;
        write!(f, "{}", self.goal)
    }
}