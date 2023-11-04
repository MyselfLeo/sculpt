use std::fmt::Display;

use crate::inductive::Formula;

pub enum Associativity {
    Left,
    Right
}


/*#[derive(Debug, Clone, Copy)]
pub enum Keyword {
    Context
}

impl Display for Keyword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Keyword::Context => write!(f, "Context"),
        }
    }
}*/

#[derive(Debug, Clone, Copy)]
pub enum Op {
    Not,
    Or,
    And,
    Implies
}

impl Op {
    pub fn from_str(op: &str) -> Option<Op> {
        match op {
            "~" => Some(Op::Not),
            "\\/" => Some(Op::Or),
            "/\\" => Some(Op::And),
            "=>" => Some(Op::Implies),
            _ => None
        }
    }

    /// Return the precedence of the operator.
    /// The greater the returned value, the higher the precedence
    /// (higher precedence = applied before)
    pub fn get_precedence(&self) -> u8 {
        match self {
            Op::Not => 3,
            Op::Or => 2,
            Op::And => 2,
            Op::Implies => 1
        }
    }


    /// Return the associativity of the operator.
    /// In the context of unary operators (like Not),
    /// Left means its a postfix operator while Right means its a prefix operator.
    pub fn get_associativity(&self) -> Associativity {
        match self {
            Op::Not => Associativity::Right,
            Op::Or => Associativity::Left,
            Op::And => Associativity::Left,
            Op::Implies => Associativity::Right
        }
    }


    /// Return the arity of the operator, i.e. its number of operands
    pub fn get_arity(&self) -> u8 {
        match self {
            Op::Not => 1,
            Op::Or => 2,
            Op::And => 2,
            Op::Implies => 2
        }
    }
}

impl Display for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Op::Not => write!(f, "~"),
            Op::Or => write!(f, "\\/"),
            Op::And => write!(f, "/\\"),
            Op::Implies => write!(f, "=>")
        }
    }
}


#[derive(Debug, Clone)]
pub enum Token {
    //Keyword(Keyword),
    Ident(String),
    Bottom,
    Op(Op),
    OpenParenthesis,
    CloseParenthesis,
}


#[derive(PartialEq, Debug)]
enum LexerStates {
    Idle,
    Op,
    Ident
}



macro_rules! op_push {
    ($buf:ident, $res:ident) => {
        match Op::from_str(&$buf.as_str()) {
            Some(op) => $res.push(Token::Op(op)),
            None => return Err(format!("Expected operator, got {} instead", $buf))
        }
        $buf.clear();
    };
}

macro_rules! ident_push {
    ($buf:ident, $res:ident) => {
        $res.push(Token::Ident($buf.clone()));
        $buf.clear();
    };
}


macro_rules! buf_push {
    ($buf:ident, $res:ident, $state:ident) => {
        if !$buf.is_empty() {
            match $state {
                LexerStates::Idle => {}
                LexerStates::Op => { op_push!($buf, $res); }
                LexerStates::Ident => { ident_push!($buf, $res); }
            }
        }
    };
}


/// Convert the string expression into a Vec of Tokens
pub fn lex(src: &str) -> Result<Vec<Token>, String> {
    let mut res: Vec<Token> = Vec::new();
    let mut buf = String::new();
    let mut state = LexerStates::Idle;

    // Idents contain only letters & numbers,
    // while operators don't contain any.

    for c in src.chars() {
        // End of token, empty buffer
        if c.is_ascii_whitespace() {
            if !buf.is_empty() {
                match state {
                    LexerStates::Idle => return Err(format!("Token {buf} not expected")),
                    LexerStates::Ident => { ident_push!(buf, res); },
                    LexerStates::Op => { op_push!(buf, res); }
                }

                state = LexerStates::Idle;
            }

            continue;
        }

        // Start or continuation of ident
        if c.is_alphanumeric() {
            // end of op token, start of ident token
            if state == LexerStates::Op { op_push!(buf, res); }

            state = LexerStates::Ident;
            buf.push(c);
        }

        // Parenthesis
        else if c == '(' || c == ')' {
            buf_push!(buf, res, state);
            state = LexerStates::Idle;

            if c == '(' {res.push(Token::OpenParenthesis);}
            else {res.push(Token::CloseParenthesis);}
        }

        // Bottom symbol
        else if c == '⊥' {
            buf_push!(buf, res, state);
            state = LexerStates::Idle;
            res.push(Token::Bottom);
        }

        // Operators
        else {
            // End of ident token, start of op token
            if state == LexerStates::Ident { ident_push!(buf, res); }

            state = LexerStates::Op;
            buf.push(c);
        }
    };


    // Push remaining of buffer
    buf_push!(buf, res, state);

    Ok(res)
}


/// Convert infix tokens to postfix tokens based on the
/// parenthesis, the operators precedence and the associativity of operators.
/// The parenthesis will get removed.
pub fn infix_to_postfix(infix: &Vec<Token>) -> Result<Vec<Token>, String> {

    let mut postfix_output: Vec<Token> = Vec::new();
    let mut stack: Vec<Token> = Vec::new();

    for t in infix {
        match t {
            //Token::Keyword(_) => {postfix_output.push(t.clone())}
            Token::Ident(_) | Token::Bottom => { postfix_output.push(t.clone())}

            Token::Op(op) => {
                match (op.get_arity(), op.get_associativity()) {
                    (1, Associativity::Left) => postfix_output.push(t.clone()),
                    (1, Associativity::Right) => stack.push(t.clone()),

                    (2, Associativity::Left) => {
                        while let Some(Token::Op(othr)) = stack.last() {
                            if othr.get_precedence() >= op.get_precedence() {
                                postfix_output.push(stack.pop().unwrap())
                            }
                            else {break;}
                        }

                        stack.push(t.clone())
                    },

                    (2, Associativity::Right) => {
                        while let Some(Token::Op(othr)) = stack.last() {
                            if othr.get_precedence() > op.get_precedence() {
                                postfix_output.push(stack.pop().unwrap())
                            }
                            else {break;}
                        }

                        stack.push(t.clone())
                    },

                    _ => return Err(format!("Unsupported operator {}", op))
                }
            }


            Token::OpenParenthesis => {stack.push(t.clone())}

            Token::CloseParenthesis => {
                while let Some(&Token::Op(_)) = stack.last() {
                    postfix_output.push(stack.pop().unwrap())
                }
                 if let Some(Token::OpenParenthesis) = stack.pop() { /* expected */ }
                 else { return Err("Invalid expression".to_string()) }
            }
        }
    }

    while let Some(t) = stack.pop() {
        postfix_output.push(t);
    }

    Ok(postfix_output)
}






/// Take postfix list of tokens and build a Formula from it
pub fn formula_from_tokens(postfix: &Vec<Token>) -> Result<Box<Formula>, String> {
    let mut formula_stack: Vec<Box<Formula>> = Vec::new();


    for token in postfix {
        let formula = match token {
            Token::Ident(id) => Formula::Variable(id.clone()),
            Token::Bottom => Formula::Bottom,
            Token::Op(op) => match op {
                Op::Not => Formula::Not(formula_stack.pop().unwrap()),
                Op::Or | Op::And | Op::Implies => {
                    let rhs = formula_stack.pop().unwrap();
                    let lhs = formula_stack.pop().unwrap();

                    match op {
                        Op::Not => unreachable!(),
                        Op::Or => Formula::Or(lhs, rhs),
                        Op::And => Formula::And(lhs, rhs),
                        Op::Implies => Formula::Implies(lhs, rhs)
                    }
                }
            },

            //Token::Keyword(kw) => { return Err(format!("Unexpected keyword '{kw}'")) },
            Token::OpenParenthesis => { return Err("Unexpected '('".to_string()) },
            Token::CloseParenthesis => { return Err("Unexpected ')'".to_string()) },
        };

        formula_stack.push(Box::new(formula));
    }


    if formula_stack.len() > 1 { return Err("Invalid expression".to_string()); }
    match formula_stack.pop() {
        Some(f) => Ok(f),
        None => Err("Invalid expression".to_string()),
    }
}