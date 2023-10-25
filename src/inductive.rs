use std::fmt::Display;

#[derive(PartialEq)]
pub enum Associativity {
    Left,
    Right
}

#[derive(Debug)]
pub enum Formula {
    Variable(String),
    Not(Box<Formula>),
    Or(Box<Formula>, Box<Formula>),
    And(Box<Formula>, Box<Formula>),
    Implies(Box<Formula>, Box<Formula>),
}

impl Formula {
    pub fn get_precedence(&self) -> u8 {
        match self {
            Formula::Variable(_) => 4,
            Formula::Not(_) => 3,
            Formula::And(_, _) | Formula::Or(_, _) => 2,
            Formula::Implies(_, _) => 1
        }
    }
}



impl Display for Formula {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let self_prcd = self.get_precedence();

        match self {
            Formula::Variable(v) => write!(f, "{}", v),

            Formula::Not(formula) => match formula.as_ref() {
                Formula::Variable(v) => write!(f, "~{}", v),
                other => write!(f, "~({})", *other)
            },

            Formula::Or(lhs, rhs) => {
                let lhs_str = if lhs.get_precedence() < self_prcd { format!("({})", lhs) }
                                     else { format!("{}", lhs) };

                let rhs_str = if rhs.get_precedence() <= self_prcd { format!("({})", rhs) }
                                     else { format!("{}", rhs) };

                write!(f, "{lhs_str} \\/ {rhs_str}")
            }

            Formula::And(lhs, rhs) => {
                let lhs_str = if lhs.get_precedence() < self_prcd { format!("({})", lhs) }
                                     else { format!("{}", lhs) };

                let rhs_str = if rhs.get_precedence() <= self_prcd { format!("({})", rhs) }
                                     else { format!("{}", rhs) };

                write!(f, "{lhs_str} /\\ {rhs_str}")
            }

            Formula::Implies(lhs, rhs) => {
                let lhs_str = if lhs.get_precedence() <= self_prcd { format!("({})", lhs) }
                                     else { format!("{}", lhs) };

                let rhs_str = if rhs.get_precedence() < self_prcd { format!("({})", rhs) }
                                     else { format!("{}", rhs) };

                write!(f, "{lhs_str} => {rhs_str}")
            }
        }
    }
}