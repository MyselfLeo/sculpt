use std::fmt::{Display, Formatter};
use crate::parser;
use crate::sequent::Sequent;
use crate::inductive::{Formula, Term};



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
    Intros,
    Trans(String),
    SplitAnd,
    And(Side, String),
    Keep(Side),
    FromOr(String),

    Generalize(String),
    FixAs(String),
    Consider(String),
    RenameAs(String),


    FromBottom,
    ExFalso(String)
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
            Rule::ExFalso(_) => write!(f, "ExFalso")
        }
    }
}


macro_rules! err_goal_form {
    ($($arg:tt)*) => {
        {
            let res = format!("The goal must be in the form {}", format!($($arg)*));
            res
        }
    };
}


impl Rule {
    pub fn apply(&self, sequent: &Sequent) -> Result<Vec<Sequent>, String> {

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
                            return Err(format!("{v} already exists"))
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
                let introduced_prop = match Formula::from_str(prop) {
                    Ok(f) => f,
                    Err(_) => return Err(format!(""))
                };

                let implication = Formula::Implies(introduced_prop.clone(), (&sequent.consequent).to_owned());

                let new_seq = vec![
                    Sequent::new(sequent.antecedents.clone(), Box::new(implication)),
                    Sequent::new(sequent.antecedents.clone(), introduced_prop)
                ];

                Ok(new_seq)
            },



            Rule::Axiom => {
                let is_axiom = sequent.antecedents.contains(&sequent.consequent);

                if is_axiom { Ok(vec![]) }
                else { Err(format!("Not an axiom")) }
            }



            Rule::And(s, prop) => {
                let introduced_prop = match Formula::from_str(prop) {
                    Ok(f) => f,
                    Err(_) => return Err(format!("Expected a formula"))
                };

                let and = match s {
                    Side::Left => Formula::And(introduced_prop, (&sequent.consequent).to_owned()),
                    Side::Right => Formula::And((&sequent.consequent).to_owned(), introduced_prop),
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
                let or = match Formula::from_str(or_prop) {
                    Ok(f) => f,
                    Err(_) => return Err(format!("Expected a formula in the form P \\/ Q"))
                };

                let (left_prop, right_prop) = match *or.to_owned() {
                    Formula::Or(lhs, rhs) => (lhs.clone(), rhs.clone()),
                    _ => return Err(format!("Expected a formula in the form P \\/ Q"))
                };

                let mut with_prop1 = sequent.antecedents.clone();
                let mut with_prop2 = sequent.antecedents.clone();
                with_prop1.push(left_prop);
                with_prop2.push(right_prop);

                let new_seq = vec![
                    Sequent::new(sequent.antecedents.clone(), or.clone()),
                    Sequent::new(with_prop1, sequent.consequent.clone()),
                    Sequent::new(with_prop2, sequent.consequent.clone()),
                ];

                Ok(new_seq)
            }




            Rule::Generalize(s) => {
                let term: Box<Term> = parser::TermParser::new().parse(s).map_err(|_| format!("Expected <Term> as <var>"))?;
                // the term must be present in the formula for it to be generalized
                if !sequent.consequent.exists(&term) {return Err(format!("{term} not present in the goal"))}

                let var = sequent.consequent.new_variable();

                let mut generalized = sequent.consequent.clone();
                generalized.rewrite(&term, &Term::Variable(var.clone()));

                let quantified = Formula::Forall(var, generalized);

                let new_seq = vec![
                    Sequent::new(sequent.antecedents.clone(), Box::new(quantified))
                ];

                Ok(new_seq)
            }


            Rule::FixAs(t) => {
                match sequent.consequent.as_ref() {

                    Formula::Exists(exists, formula) => {
                        let term: Box<Term> = parser::TermParser::new().parse(t).map_err(|_| format!("Expected <Term>"))?;
                        if sequent.consequent.exists(&term) {return Err(format!("{term} already exists"))}

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


            Rule::Consider(f) => {
                let new_form: Box<Formula> = parser::FormulaParser::new().parse(f).map_err(|_| format!("Expected exists <var>, <Formula>"))?;

                match new_form.as_ref() {
                    Formula::Exists(var, nf) => {
                        if sequent.consequent.domain().contains(&var) {return Err(format!("{var} already exists in the goal"))}
                        if sequent.domain().contains(&var) {return Err(format!("{var} already exists"))}

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

                    _ => Err(format!("Expected exists <var>, <Formula>"))
                }
            }



            Rule::RenameAs(s) => {
                let var: String = parser::VariableParser::new().parse(s).map_err(|_| format!("Expected <var>"))?;

                match sequent.consequent.as_ref() {

                    Formula::Exists(old, f) => {
                        let mut nf = Box::new(Formula::Exists(var, f.clone()));
                        nf.rewrite(&Term::Variable(old.clone()), &Term::Variable(s.clone()));
                        let mut new_s = sequent.clone();
                        new_s.consequent = nf;

                        Ok(vec![new_s])
                    }

                    Formula::Forall(old, f) => {
                        let mut nf = Box::new(Formula::Forall(var, f.clone()));
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
                        let prop = parser::FormulaParser::new().parse(prop).map_err(|_| format!("Expected <F>"))?;

                        let (true_prop, false_prop) = {
                            match *prop {
                                Formula::Not(ref ff) => (ff.clone(), prop.clone()),
                                o=> (Box::new(o.clone()), Box::new(Formula::Not(Box::new(o))))
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
