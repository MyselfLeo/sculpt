use std::fmt::{Display, Formatter};
use crate::{syntax::parser, tools};
use crate::error::Error;
use crate::syntax::lexer::Lexer;

/// First-order logic term
#[derive(Debug, Clone, PartialEq)]
pub struct Term(pub String, pub Vec<Term>);


impl Display for Term {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.1.is_empty() {
            write!(f, "{}", self.0)
        }
        else {
            write!(f, "{}({})", self.0, tools::list_str(&self.1, ", "))
        }
    }
}


impl Term {
    pub fn parse(lxr: &mut Lexer) -> Result<Term, Error> {
        parser::TermParser::new().parse(lxr).map_err(|_| Error::InvalidArguments("Invalid term".to_string()))
    }

    /// Return whether the given term is used somewhere in this term or not.
    pub fn exists(&self, term: &Term) -> bool {
        if self == term {true}
        else {
            self.1.iter().any(|t| t.exists(term))
        }
    }


    /// Replace in this term a term by another.
    pub fn rewrite(&mut self, old: &Term, new: &Term) {
        if self == old {
            *self = new.clone();
        }

        else {
            for t in &mut self.1 {
                t.rewrite(old, new)
            }
        }
    }



    /// Return a list of each variable in the domain
    /// of this Term.
    pub fn domain(&self) -> Vec<String> {
        let mut res = vec![self.0.clone()];
        res.extend(self.1.iter().flat_map(|t| t.domain()));
        res
    }
}



impl Default for Term {
    fn default() -> Self {
        Self("x".to_string(), vec![])
    }
}