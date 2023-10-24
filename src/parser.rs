use std::fmt::{Display, format};

use crate::inductive::{Formula, Op};

use super::inductive;



const INFIX_OPERATORS: [&str; 3] = ["=>", "/\\", "\\/"];
//const UNARY_OPERATORS: [&str; 1] = ["~"];
const OPERATORS: [&str; 4] = ["=>", "/\\", "\\/", "~"];

#[derive(Debug, Clone, Copy)]
pub enum Keyword {
    Context
}

impl Display for Keyword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Keyword::Context => write!(f, "Context"),
        }
    }
}




#[derive(Debug, Clone)]
pub enum Token {
    Keyword(Keyword),
    Ident(String),
    Op(String),
    OpenParenthesis,
    CloseParenthesis,
}


#[derive(PartialEq, Debug)]
enum LexerStates {
    Idle,
    Op,
    Ident
}


/// Return the precedence of the operator.
/// The greater the returned value, the higher the precedence
/// (higher precedence = applied before)
///
/// Returns None if op's precedence is not defined
fn get_precidence(op: &str) -> Option<u8> {
    match op {
        "/\\" | "\\/" => Some(2),
        "~" => Some(3),
        "=>" => Some(1),
        _ => None
    }
}




macro_rules! op_push {
    ($buf:ident, $res:ident, $state:ident) => {
        if OPERATORS.contains(&$buf.as_str()) { $res.push(Token::Op($buf.clone())) }
        else { return Err(format!("Expected operator, got {} instead", $buf)) }
        $buf.clear();
        $state = LexerStates::Idle;
    };
}

macro_rules! ident_push {
    ($buf:ident, $res:ident, $state:ident) => {
        $res.push(Token::Ident($buf.clone()));
        $buf.clear();
        $state = LexerStates::Idle;
    };
}


macro_rules! buf_push {
    ($buf:ident, $res:ident, $state:ident) => {
        if !$buf.is_empty() {
            match $state {
            LexerStates::Idle => {}
            LexerStates::Op => { op_push!($buf, $res, $state); }
            LexerStates::Ident => { ident_push!($buf, $res, $state); }
            }
        }
    };
}


/// Convert the string expression into a Vec of Tokens
pub fn lex(src: String) -> Result<Vec<Token>, String> {
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
                    LexerStates::Ident => { ident_push!(buf, res, state); },
                    LexerStates::Op => { op_push!(buf, res, state); }
                }

                state = LexerStates::Idle;
            }

            continue;
        }

        // Start or continuation of ident
        if c.is_alphanumeric() {
            // end of op token, start of ident token
            if state == LexerStates::Op { op_push!(buf, res, state); }

            state = LexerStates::Ident;
            buf.push(c);
        }

        // Parenthesis
        else if c == '(' || c == ')' {
            buf_push!(buf, res, state);
            if c == '(' {res.push(Token::OpenParenthesis);}
            else {res.push(Token::CloseParenthesis);}
        }

        // Operators
        else {
            // End of ident token, start of op token
            if state == LexerStates::Ident { ident_push!(buf, res, state); }

            state = LexerStates::Op;
            buf.push(c);
        }
    };


    // Push remaining of buffer
    buf_push!(buf, res, state);

    Ok(res)
}


/// Convert infix tokens to postfix tokens based on the
/// parenthesis and the operators precedence.
/// The parenthesis will get removed.
pub fn infix_to_postfix(infix: &Vec<Token>) -> Result<Vec<Token>, String> {

    // Shunting-Yard algorithm
    let mut postfix_output: Vec<Token> = Vec::new();
    let mut stack: Vec<Token> = Vec::new();

    for t in infix {
        match t {
            Token::Keyword(_) => {postfix_output.push(t.clone())}
            Token::Ident(_) => { postfix_output.push(t.clone())}
            Token::Op(ref crrt) => {
                let crrt_precedence = get_precidence(crrt).ok_or(format!("Unknown precedence of {crrt}"))?;

                while let Some(Token::Op(othr)) = stack.last() {
                    let othr_precedence = get_precidence(othr).ok_or(format!("Unknown precedence of {othr}"))?;

                    if othr_precedence >= crrt_precedence {
                        postfix_output.push(stack.pop().unwrap())
                    }
                    else {break;}
                }

                stack.push(t.clone())
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


// todo: recomputing of parenthesis
// if a node of the tree has a smaller precedence that its parent,
// put parenthesis around it





/// Take postfix list of tokens and build a Formula from it
pub fn formula_from_tokens(postfix: &Vec<Token>) -> Result<Box<Formula>, String> {
    let mut formula_stack: Vec<Box<Formula>> = Vec::new();


    for token in postfix {
        let formula = match token {
            Token::Ident(id) => Formula::Variable(id.clone()),
            Token::Op(op) => match op.as_str() {
                "~" => Formula::Not(formula_stack.pop().unwrap()),
                "=>" => {
                    let rhs = formula_stack.pop().unwrap();
                    let lhs = formula_stack.pop().unwrap();

                    Formula::Op(Op::Implies, lhs, rhs)
                },
                "/\\" => {
                    let rhs = formula_stack.pop().unwrap();
                    let lhs = formula_stack.pop().unwrap();

                    Formula::Op(Op::And, lhs, rhs)
                },
                "\\/" => {
                    let rhs = formula_stack.pop().unwrap();
                    let lhs = formula_stack.pop().unwrap();

                    Formula::Op(Op::Or, lhs, rhs)
                },
                _ => return Err(format!("Unsupported operator {op}"))
            },

            Token::Keyword(kw) => { return Err(format!("Unexpected keyword '{kw}'")) },
            Token::OpenParenthesis => { return Err("Unexpected '('".to_string()) },
            Token::CloseParenthesis => { return Err("Unexpected '('".to_string()) },
        };

        formula_stack.push(Box::new(formula));
    }


    if formula_stack.len() > 1 { return Err("Invalid expression".to_string()); }
    match formula_stack.pop() {
        Some(f) => Ok(f),
        None => Err("Invalid expression".to_string()),
    }
}