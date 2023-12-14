use std::fmt::Display;
use strum::IntoEnumIterator;

use crate::inductive::Formula;
use crate::rule::RuleType;


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


    /// Return a list of rules that can be applied to this sequent
    pub fn get_applicable_rules(&self) -> Vec<RuleType> {
        RuleType::iter()
            .filter(|rt| rt.is_applicable(&self))
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