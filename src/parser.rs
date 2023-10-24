// A => B        A /\ (B => C)
// => A B        /\ A => B C

const INFIX_OPERATORS: [&str; 3] = ["=>", "/\\", "\\/"];
//const UNARY_OPERATORS: [&str; 1] = ["~"];
const OPERATORS: [&str; 4] = ["=>", "/\\", "\\/", "~"];

#[derive(Debug)]
pub enum Keyword {
    Context
}


#[derive(Debug)]
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
fn get_precedence(op: &str) -> Option<u8> {
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


/// Convert infix tokens to prefix tokens based on the
/// parenthesis and the operators precedence.
/// The parenthesis will get removed.
pub fn infix_to_prefix(infix: Vec<Token>) -> Result<Vec<Token>, String> {
    let mut prefix_output: Vec<Token> = Vec::new();
    let mut stack: Vec<Token> = Vec::new();

    for t in infix {
        match t {
            Token::Keyword(_) => {unimplemented!()}
            Token::Ident(_) => {prefix_output.push(t)}
            Token::Op(_) => {}
            Token::OpenParenthesis => {stack.push(t)}
            Token::CloseParenthesis => {
                //while stack.last().unwrap() != Token::OpenParenthesis {
//
  //              }
            }
        }
    }

    todo!();
}


// todo: recomputing of parenthesis
// if a node of the tree has a smaller precedence that its parent,
// put parenthesis around it