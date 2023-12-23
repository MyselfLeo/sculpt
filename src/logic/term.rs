use std::fmt::{Display, Formatter};
use crate::{syntax::parser, tools};

/// First-order logic term, that is either a variable or a function.
/// Functions with no arguments are constants.
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
    /// Creates a new Term by parsing the given string.
    /// Each identifier (variable or function) MUST start with a lowercase letter.
    /// Note that only identifier that matches the regex r"[a-z]'*" will be considered a variable.
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