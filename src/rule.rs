use std::fmt::{Display, Formatter};
use crate::sequent::{Sequent, Hypothesis};
use crate::Formula;

pub trait Rule: Display {
    fn apply(&self, sequent: &Sequent) -> Result<Vec<Sequent>, ()>;
}


pub struct Intro {
    pub hyp_name: String
}

impl Display for Intro {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Intro")
    }
}

impl Rule for Intro {
    fn apply(&self, sequent: &Sequent) -> Result<Vec<Sequent>, ()> {
        match sequent.consequent.as_ref() {
            Formula::Implies(lhs, rhs) => {
                let mut antecedents = sequent.antecedents.clone();
                antecedents.push(Hypothesis {name: self.hyp_name.clone(), formula: lhs.to_owned()});
                Ok(vec![Sequent { antecedents, consequent: rhs.to_owned() }])
            },
            _ => Err(())
        }
    }
}