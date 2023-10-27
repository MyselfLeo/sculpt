use std::fmt::Display;

use crate::inductive::Formula;


pub struct Sequent {
    pub antecedents: Vec<Box<Formula>>,
    pub consequent: Box<Formula>
}


impl Sequent {
    // Antecedents must be named
    pub fn new(antecedents: Vec<Box<Formula>>, consequent: Box<Formula>) -> Sequent {
        Sequent { antecedents, consequent: consequent.clone() }
    }

    /// Returns a sequent with no antecedents
    pub fn from(formula: Box<Formula>) -> Sequent {
        Sequent::new(vec![], formula)
    }

    pub fn add_antecedent(&mut self, antecedent: Box<Formula>) {
        self.antecedents.push(antecedent);
    }
}

impl Display for Sequent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for formula in &self.antecedents {
            write!(f, "│ {formula}\n")?;
        }
        write!(f, "│──────────────────────────\n")?;
        write!(f, "│ {}", self.consequent)
    }
}