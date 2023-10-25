use crate::sequent::{Sequent, Hypothesis};
use crate::Formula;

pub trait Rule {
    fn apply(&self, sequent: &Sequent) -> Result<Vec<Sequent>, ()>;
}


pub struct Intro {
    pub hyp_name: String
}

impl Rule for Intro {
    fn apply(&self, sequent: &Sequent) -> Result<Vec<Sequent>, ()> {
        match sequent.goal.as_ref() {
            Formula::Implies(lhs, rhs) => {
                let mut antecedents = sequent.antecedents.clone();
                antecedents.push(Hypothesis {name: self.hyp_name.clone(), formula: lhs.to_owned()});
                Ok(vec![Sequent { antecedents, goal: rhs.to_owned() }])
            },
            _ => Err(())
        }
    }
}