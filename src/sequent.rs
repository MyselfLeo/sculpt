use std::fmt::Display;

use crate::{inductive::Formula, tools::list_str};


#[derive(Clone)]
pub struct Sequent {
    pub antecedents: Vec<Box<Formula>>,
    pub consequent: Box<Formula>
}


impl Sequent {
    // Antecedents must be named
    pub fn new(antecedents: Vec<Box<Formula>>, consequent: Box<Formula>) -> Sequent {
        Sequent { antecedents, consequent: consequent.clone() }
    }

    /// Return a list of free variables in this sequent
    pub fn domain(&self) -> Vec<String> {
        self.antecedents.iter()
            .map(|t| t.domain())
            .flatten()
            .collect()
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