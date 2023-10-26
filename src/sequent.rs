use std::fmt::Display;

use crate::inductive::Formula;

 
 #[derive(Clone)]
pub struct Hypothesis {
    pub name: String,
    pub formula: Formula
}

pub struct Sequent {
    pub antecedents: Vec<Hypothesis>,
    pub consequent: Box<Formula>
}


impl Sequent {
    // Antecedents must be named
    pub fn new(antecedents: Vec<Hypothesis>, consequent: Box<Formula>) -> Sequent {
        Sequent { antecedents, consequent: consequent.clone() }
    }

    /// Returns a sequent with no antecedents
    pub fn from(formula: Box<Formula>) -> Sequent {
        Sequent::new(vec![], formula)
    }

    pub fn add_antecedent(&mut self, antecedent: Hypothesis) {
        self.antecedents.push(antecedent);
    }
}

impl Display for Sequent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for h in &self.antecedents {
            write!(f, "│ {}: {}\n", h.name, h.formula)?;
        }
        write!(f, "│──────────────────────────\n")?;
        write!(f, "│ {}", self.consequent)
    }
}