use crate::sequent::{Sequent, Hypothesis};
use crate::Formula;

pub trait Rule {
    fn apply(&self, sequent: &Sequent) -> Result<Vec<Sequent>, ()>;
}


pub struct Intro {
    hyp_name: String
}

impl Rule for Intro {
    fn apply(&self, sequent: &Sequent) -> Result<Vec<Sequent>, ()> {
        match sequent.goal.as_ref() {
            Formula::Implies(lhs, rhs) => {
                let mut antecedents = sequent.antecedents;
                antecedents.push(Hypothesis {name: self.hyp_name, formula: *lhs});
                Ok(vec![Sequent { antecedents, goal: *rhs }])
            },
            _ => Err(())
        }
    }
}