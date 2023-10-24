pub enum Op {
    Or,
    And,
    Implies
}



pub enum Formula {
    Variable(String),
    Not(Box<Formula>),
    Op(Op, Box<Formula>, Box<Formula>)
}