use std::fmt::Display;

#[derive(Debug)]
pub enum Op {
    Or,
    And,
    Implies
}

impl Op {
    pub fn to_string(&self) -> String {
        match self {
            Op::Or => "\\/",
            Op::And => "/\\",
            Op::Implies => "=>",
        }.to_string()
    }
}


#[derive(Debug)]
pub enum Formula {
    Variable(String),
    Not(Box<Formula>),
    Op(Op, Box<Formula>, Box<Formula>)
}

impl Formula {
    pub fn get_precidence(&self) -> u8 {
        match self {
            Formula::Variable(_) => 4,
            Formula::Not(_) => 3,
            Formula::Op(Op::And | Op::Or, _, _) => 2,
            Formula::Op(Op::Implies, _, _) => 1
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

            Formula::Op(op, lhs, rhs) => {
                // Put parenthesis around lhs or rhs if needed by precidence
                let my_precidence = self.get_precidence();
                
                let lhs_string = if lhs.get_precidence() < my_precidence { format!("({})", lhs) }
                else { format!("{}", lhs) };

                let rhs_string = if rhs.get_precidence() < my_precidence { format!("({})", rhs) }
                else { format!("{}", rhs) };

                write!(f, "{lhs_string} {} {rhs_string}", op.to_string())
            },
        }
    }
}