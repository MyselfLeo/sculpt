use std::fmt::Display;

use crate::inductive::Formula;

pub enum Associativity {
    Left,
    Right
}


#[derive(Debug, Clone, Copy)]
pub enum Op {
    Not,
    Or,
    And,
    Implies,
    Forall,
    Exists
}

impl Op {
    pub fn is_op(s: &str) -> bool {
        match s {
            "~" => true,
            "\\/" => true,
            "/\\" => true,
            "=>" => true,
            "forall" => true,
            "exists" => true,
            _ => false
        }
    }


    pub fn from_str(op: &str) -> Option<Op> {
        match op {
            "~" => Some(Op::Not),
            "\\/" => Some(Op::Or),
            "/\\" => Some(Op::And),
            "=>" => Some(Op::Implies),
            "forall" => Some(Op::Forall),
            "exists" => Some(Op::Exists),
            _ => None
        }
    }

    /// Return the precedence of the operator.
    /// The greater the returned value, the higher the precedence
    /// (higher precedence = applied before)
    pub fn get_precedence(&self) -> u8 {
        match self {
            Op::Not => 4,
            Op::Or | Op::And => 3,
            Op::Implies => 2,
            Op::Forall | Op::Exists => 1
        }
    }


    /// Return the associativity of the operator.
    /// In the context of unary operators (like Not),
    /// Left means its a postfix operator while Right means its a prefix operator.
    pub fn get_associativity(&self) -> Associativity {
        match self {
            Op::Forall | Op::Exists => Associativity::Right,
            Op::Not => Associativity::Right,
            Op::Or => Associativity::Left,
            Op::And => Associativity::Left,
            Op::Implies => Associativity::Right
        }
    }


    /// Return the arity of the operator, i.e. its number of operands
    pub fn get_arity(&self) -> u8 {
        match self {
            Op::Exists | Op::Forall => 1,
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
            Op::Forall => write!(f, "forall"),
            Op::Exists => write!(f, "exists"),
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
    //Bottom,
    Op(Op),
    OpenParenthesis,
    CloseParenthesis,
    Comma
}


#[derive(PartialEq, Debug)]
enum LexerStates {
    Idle,
    Alphanumeric,
    SpecialChars
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
                LexerStates::Alphanumeric => {
                    if Op::is_op($buf.as_str()) {
                        op_push!($buf, $res);
                    }
                    else {
                        ident_push!($buf, $res);
                    }
                }
                LexerStates::SpecialChars => { op_push!($buf, $res); }
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

    let token_end_condition = |c: char| {
        c.is_ascii_whitespace()
    };

    for c in src.chars() {
        // End of token, empty buffer
        if token_end_condition(c) {
            if !buf.is_empty() {
                match state {
                    LexerStates::Idle => return Err(format!("Token {buf} not expected")),
                    _ => { buf_push!(buf, res, state); },
                }

                state = LexerStates::Idle;
            }

            continue;
        }

        // End of current token, separation
        if c == ',' {
            buf_push!(buf, res, state);
            res.push(Token::Comma);
            state = LexerStates::Idle;

            continue;
        }

        // Start or continuation of ident or alphanumeric operator (exists, forall)
        if c.is_alphanumeric() {
            // end of op token, start of alphanumeric token
            if state == LexerStates::SpecialChars { op_push!(buf, res); }

            state = LexerStates::Alphanumeric;
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
        /*else if c == '⊥' {
            buf_push!(buf, res, state);
            state = LexerStates::Idle;
            res.push(Token::Bottom);
        }*/

        // Special chars op
        else {
            // End of ident token, start of op token
            if state == LexerStates::Alphanumeric { buf_push!(buf, res, state); }

            state = LexerStates::SpecialChars;
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
            //Token::Ident(_) | Token::Bottom => { postfix_output.push(t.clone())}

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
            },

            _ => todo!()
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
            //Token::Ident(id) => Formula::Relation(id.clone()),
            Token::Ident(id) => todo!(),
            //Token::Bottom => Formula::Bottom,
            Token::Op(op) => match op {
                Op::Not => Formula::Not(formula_stack.pop().unwrap()),
                Op::Or | Op::And | Op::Implies | Op::Forall | Op::Exists => {
                    let rhs = formula_stack.pop().unwrap();
                    let lhs = formula_stack.pop().unwrap();

                    match op {
                        Op::Not => unreachable!(),
                        Op::Or => Formula::Or(lhs, rhs),
                        Op::And => Formula::And(lhs, rhs),
                        Op::Implies => Formula::Implies(lhs, rhs),
                        Op::Forall => todo!(),
                        Op::Exists => todo!(),
                    }
                }
            },

            //Token::Keyword(kw) => { return Err(format!("Unexpected keyword '{kw}'")) },
            Token::OpenParenthesis => { return Err("Unexpected '('".to_string()) },
            Token::CloseParenthesis => { return Err("Unexpected ')'".to_string()) },
            Token::Comma => { return Err("Unexpected ','".to_string()) }
        };

        formula_stack.push(Box::new(formula));
    }


    if formula_stack.len() > 1 { return Err("Invalid expression".to_string()); }
    match formula_stack.pop() {
        Some(f) => Ok(f),
        None => Err("Invalid expression".to_string()),
    }
}