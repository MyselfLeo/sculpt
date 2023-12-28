//! Natural deduction rules that can be applied to [Sequent]

use std::fmt::{Display, Formatter};
use strum::EnumIter;
use sculpt_macro::EnumType;
use crate::error::Error;
use super::{Formula, Term, Sequent};


/// Structure used by some [Rule] variants.
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

impl Default for Side {
    fn default() -> Self {
        Self::Left
    }
}


#[derive(EnumType)]
pub enum Rule {
    Axiom,
    Intro,
    Intros,
    Trans(Box<Formula>),
    SplitAnd,
    And(Side, Box<Formula>),
    Keep(Side),
    FromOr(Box<Formula>),

    Generalize(Box<Term>),
    FixAs(Box<Term>),
    Consider(Box<Formula>),
    RenameAs(String),


    FromBottom,
    ExFalso(Box<Formula>)
}

impl Display for Rule {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Rule::Intro => write!(f, "Intro"),
            Rule::Intros => write!(f, "Intros"),
            Rule::SplitAnd => write!(f, "SplitAnd"),
            Rule::Trans(s) => write!(f, "Apply {s}"),
            Rule::Axiom => write!(f, "Axiom"),
            Rule::And(s, _) => write!(f, "And {s}"),
            Rule::Keep(s) => write!(f, "Keep {s}"),
            Rule::FromOr(_) => write!(f, "FromOr"),

            Rule::Generalize(s) => write!(f, "Generalize {s}"),
            Rule::FixAs(s) => write!(f, "FixAs {s}"),
            Rule::Consider(s) => write!(f, "Consider {s}"),
            Rule::RenameAs(s) => write!(f, "Rename {s}"),

            Rule::FromBottom => write!(f, "FromBottom"),
            Rule::ExFalso(s) => write!(f, "ExFalso {s}")
        }
    }
}


macro_rules! err_goal_form {
    ($($arg:tt)*) => {
        {
            let res = format!("The goal must be in the form {}", format!($($arg)*));
            Error::CommandError(res)
        }
    };
}


impl Rule {
    /// Apply the rule to a given [Sequent]. Returns newly created sequents (0, 1 or more), or an error.
    pub fn apply(&self, sequent: &Sequent) -> Result<Vec<Sequent>, Error> {

        match self {
            Rule::Intro=> {
                // Intro can be used to introduce predicates or bound variables
                match sequent.consequent.as_ref() {
                    Formula::Implies(lhs, rhs) => {
                        let mut antecedents = sequent.antecedents.clone();
                        antecedents.push(lhs.to_owned());
                        
                        let new_seq = vec![
                            Sequent::new(antecedents, rhs.to_owned())
                        ];

                        Ok(new_seq)
                    },



                    Formula::Forall(v, f) => {
                        if sequent.domain().contains(v) {
                            return Err(Error::CommandError(format!("{v} already exists")))
                        }

                        let new_seq = vec![
                            Sequent::new(sequent.antecedents.clone(), f.to_owned())
                        ];

                        Ok(new_seq)
                    }



                    _ => Err(err_goal_form!("F => P or forall V, F"))
                }
            }



            Rule::Intros => {
                let mut seqs = vec![sequent.clone()]; // lol
                while let Ok(v) = Rule::Intro.apply(seqs.first().unwrap()) {
                    seqs = v;
                    if seqs.is_empty() {break;}
                }
                
                Ok(seqs)
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
                    _ => Err(err_goal_form!("P /\\ Q"))
                }
            }



            Rule::Trans(prop) => {
                /*let introduced_prop = match Formula::from_str(prop) {
                    Ok(f) => f,
                    Err(_) => return Err(Error::InvalidArguments(format!("Expected a formula")))
                };*/

                let implication = Formula::Implies(prop.clone(), (&sequent.consequent).to_owned());

                let new_seq = vec![
                    Sequent::new(sequent.antecedents.clone(), Box::new(implication)),
                    Sequent::new(sequent.antecedents.clone(), prop.clone())
                ];

                Ok(new_seq)
            },



            Rule::Axiom => {
                let is_axiom = sequent.antecedents.contains(&sequent.consequent);

                if is_axiom { Ok(vec![]) }
                else { Err(Error::CommandError("Not an axiom".to_string())) }
            }



            Rule::And(s, prop) => {
                let and = match s {
                    Side::Left => Formula::And(prop.clone(), (&sequent.consequent).to_owned()),
                    Side::Right => Formula::And((&sequent.consequent).to_owned(), prop.clone()),
                };

                let new_seq = vec![
                    Sequent::new(sequent.antecedents.clone(), Box::new(and))
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
                            Sequent::new(sequent.antecedents.clone(), (*kept).to_owned())
                        ];

                        Ok(new_seq)
                    },
                    _ => Err(err_goal_form!("P \\/ Q"))
                }
            }



            Rule::FromOr(or_prop) => {
                let (left_prop, right_prop) = match *or_prop.clone() {
                    Formula::Or(lhs, rhs) => (lhs.clone(), rhs.clone()),
                    _ => return Err(Error::InvalidArguments("Expected a formula in the form P \\/ Q".to_string()))
                };

                let mut with_prop1 = sequent.antecedents.clone();
                let mut with_prop2 = sequent.antecedents.clone();
                with_prop1.push(left_prop);
                with_prop2.push(right_prop);

                let new_seq = vec![
                    Sequent::new(sequent.antecedents.clone(), or_prop.clone()),
                    Sequent::new(with_prop1, sequent.consequent.clone()),
                    Sequent::new(with_prop2, sequent.consequent.clone()),
                ];

                Ok(new_seq)
            }




            Rule::Generalize(term) => {
                // the term must be present in the formula for it to be generalized
                if !sequent.consequent.exists(&term) {return Err(Error::CommandError(format!("{term} not present in the goal")))}

                let var = sequent.consequent.new_variable();

                let mut generalized = sequent.consequent.clone();
                generalized.rewrite(&term, &Term::Variable(var.clone()));

                let quantified = Formula::Forall(var, generalized);

                let new_seq = vec![
                    Sequent::new(sequent.antecedents.clone(), Box::new(quantified))
                ];

                Ok(new_seq)
            }


            Rule::FixAs(term) => {
                match sequent.consequent.as_ref() {

                    Formula::Exists(exists, formula) => {
                        if sequent.consequent.exists(&term) {return Err(Error::InvalidArguments(format!("{term} already exists")))}

                        let mut fixed = formula.clone();
                        fixed.rewrite(&Term::Variable(exists.clone()), &term);

                        let new_seq = vec![
                            Sequent::new(sequent.antecedents.clone(), fixed)
                        ];

                        Ok(new_seq)
                    }

                    _ => Err(err_goal_form!("exists <V>, <F>"))

                }
            }


            Rule::Consider(new_form) => {
                match new_form.as_ref() {
                    Formula::Exists(var, nf) => {
                        if sequent.consequent.domain().contains(&var) {return Err(Error::CommandError(format!("{var} already exists in the goal")))}
                        if sequent.domain().contains(&var) {return Err(Error::CommandError(format!("{var} already exists")))}

                        let mut with_nf = sequent.clone();
                        with_nf.antecedents.push(nf.clone());

                        let mut goal_nf = sequent.clone();
                        goal_nf.consequent = new_form.clone();

                        let new_seq = vec![
                            goal_nf,
                            with_nf
                        ];

                        Ok(new_seq)
                    }

                    _ => Err(Error::InvalidArguments(format!("Expected exists <var>, <Formula>")))
                }
            }



            Rule::RenameAs(s) => {
                match sequent.consequent.as_ref() {

                    Formula::Exists(old, f) => {
                        let mut nf = Box::new(Formula::Exists(s.clone(), f.clone()));
                        nf.rewrite(&Term::Variable(old.clone()), &Term::Variable(s.clone()));
                        let mut new_s = sequent.clone();
                        new_s.consequent = nf;

                        Ok(vec![new_s])
                    }

                    Formula::Forall(old, f) => {
                        let mut nf = Box::new(Formula::Forall(s.clone(), f.clone()));
                        nf.rewrite(&Term::Variable(old.clone()), &Term::Variable(s.clone()));
                        let mut new_s = sequent.clone();
                        new_s.consequent = nf;

                        Ok(vec![new_s])
                    }

                    _ => Err(err_goal_form!("exists <V>, <F> OR forall <V>, <F>"))
                }
            }



            Rule::FromBottom => {
                // invert current formula
                let new_prop = match sequent.consequent.as_ref() {
                    Formula::Not(e) => e.clone(),
                    e => Box::new(Formula::Not(Box::new(e.clone())))
                };

                let mut with_prop = sequent.antecedents.clone();
                with_prop.push(new_prop);

                let new_seq = vec![
                    Sequent::new(with_prop, Box::new(Formula::Falsum))
                ];

                Ok(new_seq)
            }



            Rule::ExFalso(prop) => {
                // ExFalso only works if current consequent is Bottom (i.e false)
                match sequent.consequent.as_ref() {
                    Formula::Falsum => {
                        let (true_prop, false_prop) = {
                            match *prop.clone() {
                                Formula::Not(ref ff) => (ff.clone(), prop.clone()),
                                o => (Box::new(o.clone()), Box::new(Formula::Not(Box::new(o))))
                            }
                        };
        
                        let new_seq = vec![
                            Sequent::new(sequent.antecedents.clone(), true_prop),
                            Sequent::new(sequent.antecedents.clone(), false_prop)
                        ];
        
                        Ok(new_seq)
                    },
                    _ => Err(err_goal_form!("falsum"))
                }
            }
        }
    }
}







impl RuleType {
    /// Return whether the rule can be applied to a given sequent.
    pub fn is_applicable(&self, sequent: &Sequent) -> bool {
        match self {
            RuleType::Axiom => {
                sequent.antecedents.contains(&sequent.consequent)
            }
            RuleType::Intro | RuleType::Intros => {
                if let &Formula::Implies(_, _) = &sequent.consequent.as_ref() { true }
                else if let &Formula::Forall(_, _) = &sequent.consequent.as_ref() { true }
                else { false }
            }
            RuleType::Trans => true,
            RuleType::SplitAnd => {
                if let &Formula::And(_, _) = &sequent.consequent.as_ref() { true }
                else { false }
            }
            RuleType::And => true,
            RuleType::Keep => {
                if let &Formula::Or(_, _) = &sequent.consequent.as_ref() { true }
                else { false }
            }
            RuleType::FromOr => true,
            RuleType::Generalize => true,
            RuleType::FixAs => {
                if let &Formula::Exists(_, _) = &sequent.consequent.as_ref() { true }
                else { false }
            }
            RuleType::Consider => true,
            RuleType::RenameAs => {
                if let &Formula::Forall(_, _) = &sequent.consequent.as_ref() { true }
                else if let &Formula::Exists(_, _) = &sequent.consequent.as_ref() { true }
                else { false }
            }
            RuleType::FromBottom => true,
            RuleType::ExFalso=> {
                if let &Formula::Falsum = &sequent.consequent.as_ref() { true }
                else { false }
            }
        }
    }
}