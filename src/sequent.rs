use std::fmt::Display;

use crate::{inductive::Formula, tools::list_str};


#[derive(Clone)]
pub struct Sequent {
    pub antecedents: Vec<Box<Formula>>,
    pub consequent: Box<Formula>,
    pub bound_variables: Vec<String>,
}


impl Sequent {
    // Antecedents must be named
    pub fn new(antecedents: Vec<Box<Formula>>, consequent: Box<Formula>, bound_variables: Vec<String>) -> Sequent {
        Sequent { antecedents, consequent: consequent.clone(), bound_variables }
    }
}

impl Display for Sequent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.bound_variables.len() > 0 {
            let bound = list_str(&self.bound_variables, ", ");
        write!(f, "│ bound: {bound}\n")?;
        }

        for formula in &self.antecedents {
            write!(f, "│ {formula}\n")?;
        }
        write!(f, "│──────────────────────────\n")?;
        write!(f, "│ {}", self.consequent)
    }
}