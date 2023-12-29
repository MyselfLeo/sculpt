use std::fmt::Display;
use strum::IntoEnumIterator;

use crate::logic::Formula;
use crate::logic::rule::RuleType;


#[derive(Clone, Debug)]
pub struct Sequent {
    pub antecedents: Vec<Formula>,
    pub consequent: Box<Formula>
}


impl Sequent {
    // Antecedents must be named
    pub fn new(antecedents: Vec<Formula>, consequent: Box<Formula>) -> Sequent {
        Sequent { antecedents, consequent: consequent.clone() }
    }

    /// Return a list of free variables in this sequent
    pub fn domain(&self) -> Vec<String> {
        self.antecedents.iter()
            .flat_map(|t| t.domain())
            .collect()
    }


    /// Return a list of rules that can be applied to this sequent
    pub fn get_applicable_rules(&self) -> Vec<RuleType> {
        RuleType::iter()
            .filter(|rt| rt.is_applicable(self))
            .collect()
    }
}

impl Display for Sequent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for formula in &self.antecedents {
            writeln!(f, "│ {formula}")?;
        }
        writeln!(f, "│──────────────────────────")?;
        writeln!(f, "│ {}", self.consequent)
    }
}