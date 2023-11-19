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
    pub fn from_str(str: &str) -> Result<Box<Term>, String> {
        parser::TermParser::new().parse(str).map_err(|_| "Invalid term".to_string())
    }

    /// Return whether the given term is used somewhere in this term or not.
    pub fn exists(&self, term: &Term) -> bool {
        if self == term {true}
        else {
            match self {
                Term::Variable(_) => false,
                Term::Function(_, terms) => {
                    terms.iter().any(|t| t.exists(term))
                }
            }
        }
    }


    /// Replace in this term a term by another.
    pub fn rewrite(&mut self, old: &Term, new: &Term) {
        if self == old {
            println!("h");
            *self = new.clone();
        }

        else {
            match self {
                Term::Function(_, terms) => {
                    for t in terms {
                        t.rewrite(old, new)
                    }
                }
                _ => ()
            }
        }
    }



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


    /// Return whether the given term is used somewhere in this formula or not.
    pub fn exists(&self, term: &Term) -> bool {
        match self {
            Formula::Relation(_, terms) => terms.iter().any(|t| t.exists(term)),
            Formula::Not(f) => f.exists(term),
            Formula::Or(f1, f2) => f1.exists(term) || f2.exists(term),
            Formula::And(f1, f2) => f1.exists(term) || f2.exists(term),
            Formula::Implies(f1, f2) => f1.exists(term) || f2.exists(term),
            Formula::Forall(_, f) => f.exists(term),
            Formula::Exists(_, f) => f.exists(term),
        }
    }


    /// Replace in this formula a term by another.
    pub fn rewrite(&mut self, old: &Term, new: &Term) {
        match self {
            Formula::Relation(_, terms) => {
                for t in terms {
                    t.rewrite(old, new)
                }
            },
            Formula::Not(f) => f.rewrite(old, new),
            Formula::Or(f1, f2) => {
                f1.rewrite(old, new);
                f2.rewrite(old, new);
            },
            Formula::And(f1, f2) => {
                f1.rewrite(old, new);
                f2.rewrite(old, new);
            },
            Formula::Implies(f1, f2) => {
                f1.rewrite(old, new);
                f2.rewrite(old, new);
            },
            Formula::Forall(_, f)  | Formula::Exists(_, f)=> {
                f.rewrite(old, new)
            }
        }
    }

    
    /// Return a list of the variables used in this formula
    pub fn domain(&self) -> Vec<String> {
        match self {
            Formula::Relation(_, t) => {
                t.iter()
                 .map(|t| t.domain())
                 .flatten()
                 .collect()
            },

            Formula::Exists(v, f) | Formula::Forall(v, f) => {
                let mut subdomain = f.domain();
                if !subdomain.contains(v) {subdomain.push(v.to_string());}

                subdomain
            },

            Formula::Not(f) => f.domain(),
            Formula::Or(f1, f2) | Formula::And(f1, f2) | Formula::Implies(f1, f2) => {
                let mut domain = f1.domain();
                domain.append(&mut f2.domain());
                domain.sort();
                domain.dedup();

                domain
            },
        }
    }


    /// Return a new variable.
    /// It will first try variables from x to z then w to a,
    /// then add a ' and repeat until found.
    pub fn new_variable(&self) -> String {
        let mut prime: u8 = 0;
        let existing = self.domain();

        let with_primes = |c: char, p: u8| {
            let mut res = String::new();
            res.push(c);
            for _ in 0..p {res.push('\'')}
            res
        };

        loop {
            for c in ('x'..='z').chain(('w'..='a').rev()) {
                if !existing.contains(&with_primes(c, prime)) {
                    return with_primes(c, prime)
                }
            }

            prime += 1;
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