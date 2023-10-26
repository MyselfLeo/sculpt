use std::fmt::{Display, Formatter};
use std::ops::Deref;
use crate::sequent::{Sequent, Hypothesis};
use crate::Formula;





pub enum Rule {
    Intro(String),
    SplitAnd,
    Elim(String),
    Axiom
}

impl Display for Rule {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Rule::Intro(_) => write!(f, "Intro"),
            Rule::SplitAnd => write!(f, "SplitAnd"),
            Rule::Elim(s) => write!(f, "Apply {s}"),
            Rule::Axiom => write!(f, "Axiom")
        }
    }
}

impl Rule {
    pub fn apply(&self, sequent: &Sequent) -> Result<Vec<Sequent>, ()> {

        match self {
            Rule::Intro(hyp_name) => {
                match sequent.consequent.as_ref() {
                    Formula::Implies(lhs, rhs) => {
                        let mut antecedents = sequent.antecedents.clone();
                        antecedents.push(Hypothesis {name: hyp_name.clone(), formula: *lhs.to_owned() });
                        Ok(vec![Sequent { antecedents, consequent: rhs.to_owned() }])
                    },
                    _ => Err(())
                }
            }



            Rule::SplitAnd => {
                match sequent.consequent.as_ref() {
                    Formula::And(lhs, rhs) => {
                        let new_seq = vec![
                            Sequent::new(sequent.antecedents.clone(), lhs.to_owned()),
                            Sequent::new(sequent.antecedents.clone(), rhs.to_owned())
                        ];

                        Ok(new_seq)
                    },
                    _ => Err(())
                }
            }



            Rule::Elim(prop) => {
                let eliminated_prop = match Formula::from_str(prop) {
                    Ok(f) => f,
                    Err(_) => return Err(())
                };

                let implication = Formula::Implies(eliminated_prop.clone(), (&sequent.consequent).to_owned());

                let new_seq = vec![
                    Sequent::new(sequent.antecedents.clone(), Box::new(implication)),
                    Sequent::new(sequent.antecedents.clone(), eliminated_prop)
                ];

                Ok(new_seq)
            },



            Rule::Axiom => {
                let is_axiom = sequent.antecedents
                    .iter()
                    .find(|h| &h.formula == sequent.consequent.deref())
                    .is_some();

                if is_axiom { Ok(vec![]) }
                else { Err(()) }
            }
        }
    }
}
