use std::fmt::{Display, Formatter};
use crate::sequent::Sequent;
use crate::inductive::Formula;



pub enum Side {
    Left,
    Right
}

impl Display for Side {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Side::Left => write!(f, "Left"),
            Side::Right => write!(f, "Right"),
        }
    }
}


pub enum Rule {
    Axiom,
    Intro,
    Trans(String),
    SplitAnd,
    And(Side, String),
    Keep(Side),
    FromOr(String),

    Generalize(String),

    FromBottom,
    ExFalso(String)
}

impl Display for Rule {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Rule::Intro => write!(f, "Intro"),
            Rule::SplitAnd => write!(f, "SplitAnd"),
            Rule::Trans(s) => write!(f, "Apply {s}"),
            Rule::Axiom => write!(f, "Axiom"),
            Rule::And(s, _) => write!(f, "And {s}"),
            Rule::Keep(s) => write!(f, "Keep {s}"),
            Rule::FromOr(_) => write!(f, "FromOr"),

            Rule::Generalize(s) => write!(f, "Generalize {s}"),

            Rule::FromBottom => write!(f, "FromBottom"),
            Rule::ExFalso(_) => write!(f, "ExFalso")
        }
    }
}

impl Rule {
    pub fn apply(&self, sequent: &Sequent) -> Result<Vec<Sequent>, ()> {

        match self {
            Rule::Intro=> {
                // Intro can be used to introduce predicates or bound variables
                match sequent.consequent.as_ref() {
                    Formula::Implies(lhs, rhs) => {
                        let mut antecedents = sequent.antecedents.clone();
                        antecedents.push(lhs.to_owned());
                        
                        let new_seq = vec![
                            Sequent::new(antecedents, rhs.to_owned(), sequent.bound_variables.clone())
                        ];

                        Ok(new_seq)
                    },



                    Formula::Forall(v, f) => {
                        let mut bound = sequent.bound_variables.clone();
                        if bound.contains(v) {return Err(())}
                        bound.push(v.clone());

                        let new_seq = vec![
                            Sequent::new(sequent.antecedents.clone(), f.to_owned(), bound)
                        ];

                        Ok(new_seq)
                    }



                    _ => Err(())
                }
            }



            Rule::SplitAnd => {
                match sequent.consequent.as_ref() {
                    Formula::And(lhs, rhs) => {
                        let new_seq = vec![
                            Sequent::new(sequent.antecedents.clone(), lhs.to_owned(), sequent.bound_variables.clone()),
                            Sequent::new(sequent.antecedents.clone(), rhs.to_owned(), sequent.bound_variables.clone())
                        ];

                        Ok(new_seq)
                    },
                    _ => Err(())
                }
            }



            Rule::Trans(prop) => {
                let introduced_prop = match Formula::from_str(prop) {
                    Ok(f) => f,
                    Err(_) => return Err(())
                };

                let implication = Formula::Implies(introduced_prop.clone(), (&sequent.consequent).to_owned());

                let new_seq = vec![
                    Sequent::new(sequent.antecedents.clone(), Box::new(implication), sequent.bound_variables.clone()),
                    Sequent::new(sequent.antecedents.clone(), introduced_prop, sequent.bound_variables.clone())
                ];

                Ok(new_seq)
            },



            Rule::Axiom => {
                let is_axiom = sequent.antecedents.contains(&sequent.consequent);

                if is_axiom { Ok(vec![]) }
                else { Err(()) }
            }



            Rule::And(s, prop) => {
                let introduced_prop = match Formula::from_str(prop) {
                    Ok(f) => f,
                    Err(_) => return Err(())
                };

                let and = match s {
                    Side::Left => Formula::And(introduced_prop, (&sequent.consequent).to_owned()),
                    Side::Right => Formula::And((&sequent.consequent).to_owned(), introduced_prop),
                };

                let new_seq = vec![
                    Sequent::new(sequent.antecedents.clone(), Box::new(and), sequent.bound_variables.clone())
                ];

                Ok(new_seq)
            }



            Rule::Keep(s) => {
                match sequent.consequent.as_ref() {
                    Formula::Or(lhs, rhs) => {

                        let kept = match s {
                            Side::Left => lhs,
                            Side::Right => rhs
                        };

                        let new_seq = vec![
                            Sequent::new(sequent.antecedents.clone(), (*kept).to_owned(), sequent.bound_variables.clone())
                        ];

                        Ok(new_seq)
                    },
                    _ => Err(())
                }
            }



            Rule::FromOr(or_prop) => {
                let or = match Formula::from_str(or_prop) {
                    Ok(f) => f,
                    Err(_) => return Err(())
                };

                let (left_prop, right_prop) = match *or.to_owned() {
                    Formula::Or(lhs, rhs) => (lhs.clone(), rhs.clone()),
                    _ => return Err(())
                };

                let mut with_prop1 = sequent.antecedents.clone();
                let mut with_prop2 = sequent.antecedents.clone();
                with_prop1.push(left_prop);
                with_prop2.push(right_prop);

                let new_seq = vec![
                    Sequent::new(sequent.antecedents.clone(), or.clone(), sequent.bound_variables.clone()),
                    Sequent::new(with_prop1, sequent.consequent.clone(), sequent.bound_variables.clone()),
                    Sequent::new(with_prop2, sequent.consequent.clone(), sequent.bound_variables.clone()),
                ];

                Ok(new_seq)
            }




            Rule::Generalize(term) => {
                todo!()
            }




            Rule::FromBottom => {
                unimplemented!()
                /*// invert current formula
                let new_prop = match sequent.consequent.as_ref() {
                    Formula::Not(e) => e.clone(),
                    e => Box::new(Formula::Not(Box::new(e.clone())))
                };

                let mut with_prop = sequent.antecedents.clone();
                with_prop.push(new_prop);

                let new_seq = vec![
                    Sequent::new(with_prop, Box::new(Formula::Bottom))
                ];

                Ok(new_seq)*/
            }



            Rule::ExFalso(_) => {
                unimplemented!()
                // ExFalso only works if current consequent is Bottom (i.e false)
               /* match sequent.consequent.as_ref() {
                    Formula::Bottom => {},
                    _ => return Err(())
                };

                let (true_prop, false_prop) = match Formula::from_str(prop) {
                    Ok(f) => {
                        match *f {
                            Formula::Not(ref ff) => (ff.clone(), f.clone()),
                            o=> (Box::new(o.clone()), Box::new(Formula::Not(Box::new(o))))
                        }
                    },
                    Err(_) => return Err(())
                };


                let new_seq = vec![
                    Sequent::new(sequent.antecedents.clone(), true_prop),
                    Sequent::new(sequent.antecedents.clone(), false_prop)
                ];

                Ok(new_seq)*/
            }
        }
    }
}
