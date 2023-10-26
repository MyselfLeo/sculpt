use std::fmt::Display;

use crate::parser;

#[derive(PartialEq)]
pub enum Associativity {
    Left,
    Right
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub enum Formula {
    Variable(String),
    Not(Box<Formula>),
    Or(Box<Formula>, Box<Formula>),
    And(Box<Formula>, Box<Formula>),
    Implies(Box<Formula>, Box<Formula>),
}

impl Formula {
    pub fn from_str(str: &str) -> Result<Box<Formula>, String> {
        let tokens = parser::lex(str)?;
        let postfix = parser::infix_to_postfix(&tokens)?;
        parser::formula_from_tokens(&postfix)
    }

    pub fn get_precedence(&self) -> u8 {
        match self {
            Formula::Variable(_) => 4,
            Formula::Not(_) => 3,
            Formula::And(_, _) | Formula::Or(_, _) => 2,
            Formula::Implies(_, _) => 1
        }
    }

    pub fn get_op_symbol(&self) -> &'static str {
        match self {
            Formula::Variable(_) => "",
            Formula::Not(_) => "~",
            Formula::Or(_, _) => "\\/",
            Formula::And(_, _) => "/\\",
            Formula::Implies(_, _) => "=>"
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
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Formula::Variable(v) => write!(f, "{}", v),

            Formula::Not(formula) => match formula.as_ref() {
                Formula::Variable(v) => write!(f, "~{}", v),
                other => write!(f, "~({})", *other)
            },

            Formula::Or(lhs, rhs) => display_binary_left!(self, lhs, rhs, f),
            Formula::And(lhs, rhs) => display_binary_left!(self, lhs, rhs, f),
            Formula::Implies(lhs, rhs) => display_binary_right!(self, lhs, rhs, f)
        }
    }
}