use std::fmt::{Display, Formatter};

use crate::{tools, parser};



#[derive(Debug, Clone, PartialEq)]
pub enum Term {
    Variable(String),
    Function(String, Vec<Box<Term>>)
}


impl Display for Term {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Term::Variable(v) => write!(f, "{v}"),
            Term::Function(v, t) => {
                if t.len() == 0 {
                    write!(f, "{v}")
                }
                else {
                    write!(f, "{v}({})", tools::list_str(t, ", "))
                }
            }
        }
    }
}


impl Term {
    /// Return a list of each variable in the domain
    /// of this Term.
    pub fn domain(&self) -> Vec<String> {
        match self {
            Term::Variable(x) => vec![x.clone()],
            Term::Function(_, terms) => {
                terms.iter()
                    .map(|t| t.domain())
                    .flatten()
                    .collect()
            }
        }
    }
}



#[derive(Debug, Clone, PartialEq)]
pub enum Formula {
    Relation(String, Vec<Box<Term>>),
    Not(Box<Formula>),
    Or(Box<Formula>, Box<Formula>),
    And(Box<Formula>, Box<Formula>),
    Implies(Box<Formula>, Box<Formula>),
    Forall(String, Box<Formula>),
    Exists(String, Box<Formula>)
}

impl Formula {
    pub fn from_str(str: &str) -> Result<Box<Formula>, String> {
        parser::FormulaParser::new().parse(str).map_err(|_| "Invalid formula".to_string())
    }

    pub fn get_precedence(&self) -> u8 {
        match self {
            //Formula::Bottom => 4,
            Formula::Relation(_, _) => 5,
            Formula::Not(_) => 4,
            Formula::And(_, _) | Formula::Or(_, _) => 3,
            Formula::Implies(_, _) => 2,
            Formula::Forall(_, _) | Formula::Exists(_, _) => 1
        }
    }

    pub fn get_op_symbol(&self) -> &'static str {
        match self {
            //Formula::Bottom => "",
            Formula::Relation(_, _) => "",
            Formula::Not(_) => "~",
            Formula::Or(_, _) => "\\/",
            Formula::And(_, _) => "/\\",
            Formula::Implies(_, _) => "=>",
            Formula::Forall(_, _) => "forall",
            Formula::Exists(_, _) => "exists"
        }
    }

    /// Return respectively the free and bound variables of the formula.
    /// If a variable is both free and bound (ex: P(x) \/ forall x, Y),
    /// it will be considered free in the overall formula.
    pub fn domain(&self) -> (Vec<String>, Vec<String>) {
        match self {
            Formula::Relation(_, t) => {
                let free = t.iter()
                            .map(|t| t.domain())
                            .flatten()
                            .collect();

                (free, vec![])
            },

            Formula::Exists(v, f) | Formula::Forall(v, f) => {
                let mut res = f.domain();
                res.1.push(v.to_string());
                res
            },

            Formula::Not(f) => f.domain(),
            Formula::Or(f1, f2) => {
                let mut new = (vec![], vec![]);
                let mut d1 = f1.domain();
                let mut d2 = f2.domain();

                new.0.append(&mut d1.0);
                new.0.append(&mut d2.0);

                for v in d1.1 {
                    if !new.0.contains(&v) {new.1.push(v)}
                }
                for v in d2.1 {
                    if !new.0.contains(&v) {new.1.push(v)}
                }

                new
            },
            Formula::And(_, _) => todo!(),
            Formula::Implies(_, _) => todo!(),
        }
    }
}



macro_rules! display_binary_left {
    ($self:ident, $lhs:ident, $rhs:ident, $f:ident) => {
        {
            let lhs_str = if $lhs.get_precedence() < $self.get_precedence() { format!("({})", $lhs) }
                      else { format!("{}", $lhs) };
            let rhs_str = if $rhs.get_precedence() <= $self.get_precedence() { format!("({})", $rhs) }
                          else { format!("{}", $rhs) };

            write!($f, "{lhs_str} {} {rhs_str}", $self.get_op_symbol())
        }
    }
}

macro_rules! display_binary_right {
    ($self:ident, $lhs:ident, $rhs:ident, $f:ident) => {
        {
            let lhs_str = if $lhs.get_precedence() <= $self.get_precedence() { format!("({})", $lhs) }
                      else { format!("{}", $lhs) };
            let rhs_str = if $rhs.get_precedence() < $self.get_precedence() { format!("({})", $rhs) }
                          else { format!("{}", $rhs) };

            write!($f, "{lhs_str} {} {rhs_str}", $self.get_op_symbol())
        }
    }
}


impl Display for Formula {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            //Formula::Bottom => write!(f, "⊥"),

            Formula::Relation(v, t) => {
                if t.len() == 0 {
                    write!(f, "{v}")
                }
                else {
                    write!(f, "{v}({})", tools::list_str(t, ", "))
                }
            }

            Formula::Not(formula) => match formula.as_ref() {
                Formula::Relation(v, t) => {
                    if v.len() == 0 {
                        write!(f, "~{}", v)
                    }
                    else {
                        write!(f, "~({}({}))", v, tools::list_str(t, ", "))
                    }
                },
                other => write!(f, "~({})", *other)
            },

            Formula::Or(lhs, rhs) => display_binary_left!(self, lhs, rhs, f),
            Formula::And(lhs, rhs) => display_binary_left!(self, lhs, rhs, f),
            Formula::Implies(lhs, rhs) => display_binary_right!(self, lhs, rhs, f),
            Formula::Forall(v, p) => write!(f, "forall {v}, {p}"),
            Formula::Exists(v, p) => write!(f, "exists {v}, {p}")
        }
    }
}