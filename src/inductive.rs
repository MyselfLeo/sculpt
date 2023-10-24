#[derive(Debug)]
pub enum Op {
    Or,
    And,
    Implies
}


#[derive(Debug)]
pub enum Formula {
    Variable(String),
    Not(Box<Formula>),
    Op(Op, Box<Formula>, Box<Formula>)
}