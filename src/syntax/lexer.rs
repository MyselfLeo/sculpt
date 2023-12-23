//! Custom lexer passed to LALRPOP to parse the commands.
//! The need of a custom lexer comes from the fact that some tokens may have different semantics
//! depending on the state of the interpreter, notably identifiers which needs to have their type
//! defined (terms, propositions, etc).
//! Note that this parser does not parse REPL-specific commands (exit, help, etc). Those are managed
//! by the REPL itself.

use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};
use std::hash::Hash;

// Those may be expanded by code later (extensions, custom rule definition maybe?)
const DEFAULT_KEYWORDS: [&str; 4] = ["Def", "Thm", "Admit", "Qed"];
const DEFAULT_RULES: [&str; 16] = [
    "axiom",
    "intro",
    "intros",
    "trans",
    "split",
    "and_left",
    "and_right",
    "keep_left",
    "keep_right",
    "from_or",
    "gen",
    "fix_as",
    "consider",
    "rename_as",
    "from_bottom",
    "exfalso"
];
const SYMBOLS: [&str; 9] = [
    "~",
    "=>",
    "\\/",
    "/\\",
    "(",
    ")",
    ",",
    ".",
    "::"
];


const COMMENT_START: &str = "//";


/// Returns true if c is one of the following:
/// - alphabetic
/// - alphanumeric
/// - '_'
pub fn is_ident_allowed(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}


pub enum ParserError {
    UnknownToken(Spanned<String, usize>)
}

impl Debug for ParserError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ParserError::UnknownToken(s) => write!(f, "Unknown token '{}'", s.1)
        }
    }
}

pub type Spanned<Token, Loc> = (Loc, Token, Loc);


#[derive(Debug)]
pub enum Token {
    Keyword(String),    // Keyword
    RuleName(String),   // Known rule name
    Term(String),       // Known term identifier
    Relation(String),   // Known relation identifier
    Ident(String),      // Unknown identifier (new term/relation, thm name, etc.)

    Wave,               // ~
    DoubleArrow,        // =>
    Or,                 // \/
    And,                // /\
    OpenParen,          // (
    CloseParen,         // )
    Comma,              // ,
    Dot,                // .
    DoubleColon         // ::
}



pub struct Context {
    pub terms: HashMap<String, usize>,
    pub relations: HashMap<String, usize>
}

impl Context {
    pub fn new() -> Context {
        Context { terms: HashMap::new(), relations: HashMap::new() }
    }
}

#[derive(Debug)]
enum BufState {
    AlphaNum,
    Sym,
    Idle
}

pub struct Lexer {
    pub tokens: Vec<Spanned<Token, usize>>
}

impl Lexer {

    fn token_from_str(buf: &String, buf_state: &BufState, context: &Context) -> Option<Token> {
        let res = match buf_state {
            BufState::Idle => unreachable!(),
            BufState::AlphaNum => {
                if DEFAULT_KEYWORDS.contains(&buf.as_str()) {
                    Token::Keyword(buf.to_string())
                }
                else if DEFAULT_RULES.contains(&buf.as_str()) {
                    Token::RuleName(buf.clone())
                }
                else if context.terms.contains_key(buf) {
                    Token::Term(buf.clone())
                }
                else if context.relations.contains_key(buf) {
                    Token::Relation(buf.clone())
                }
                else {
                    Token::Ident(buf.clone())
                }
            }

            BufState::Sym => {
                match buf.as_str() {
                    "~" => Token::Wave,
                    "=>" => Token::DoubleArrow,
                    "\\/" => Token::Or,
                    "/\\" => Token::And,
                    "(" => Token::OpenParen,
                    ")" => Token::CloseParen,
                    "," => Token::Comma,
                    "." => Token::Dot,
                    "::" => Token::DoubleColon,
                    e => return None
                }
            }
        };

        Some(res)
    }


    pub fn lex(input: &str, context: &Context) -> Result<Self, ParserError> {
        let mut buf_state = BufState::Idle;
        let mut buf = String::new();
        let mut buf_start: usize = 0;
        let mut line_skip = false;

        let mut tokens: Vec<Spanned<Token, usize>> = vec![];

        'char_iter: for (i, c) in input.char_indices() {
            macro_rules! push_buf {
                () => {
                    match Self::token_from_str(&buf, &buf_state, context) {
                    None => {return Err(ParserError::UnknownToken((buf_start, buf.clone(), i-1)))},
                    Some(t) => {
                        tokens.push((buf_start, t, i-1));
                        buf.clear();
                        buf_start = i;
                    }
                }
                };
            }

            if c == '\n' {
                line_skip = false;
                continue
            }

            if line_skip { continue }

            // When the start of a comment is encountered, clean current buf and return early
            if buf == COMMENT_START {
                buf.clear();
                buf_state = BufState::Idle;
                line_skip = true;
                continue 'char_iter;
            }

            if SYMBOLS.contains(&buf.as_str()) {
                push_buf!();
                buf.clear();
                buf_start = i;
                buf_state = BufState::Idle;
            }


            if c.is_whitespace() {
                if !buf.is_empty() {
                    push_buf!();
                    buf.clear();
                }
                buf_start = i+1;
                buf_state = BufState::Idle;
                continue 'char_iter;
            }

            // buf state is used to allow alphanumeric tokens (idents) and symbolic tokens
            // (operators) to be written with no space between them
            if is_ident_allowed(c) {
                match buf_state {
                    BufState::Idle => buf_start = i,
                    BufState::AlphaNum => (),
                    BufState::Sym => {
                        push_buf!();
                        buf_start = i;
                    }
                }
                buf_state = BufState::AlphaNum;
            }
            else {
                match buf_state {
                    BufState::Idle => buf_start = i,
                    BufState::Sym => (),
                    BufState::AlphaNum => {
                        push_buf!();
                        buf_start = i;
                    }
                }
                buf_state = BufState::Sym;
            }


            buf.push(c);
        }


        if !buf.is_empty() {
            match Self::token_from_str(&buf, &buf_state, context) {
                None => {
                    return Err(ParserError::UnknownToken((buf_start, buf.clone(), input.len() - 1)))
                },
                Some(t) => {
                    tokens.push((buf_start, t, input.len() - 1));
                }
            };
        }
        //push_buf(&buf, buf_start, input.len()-1, &mut tokens)?;

        Ok(Lexer{tokens})
    }
}



/*
impl IntoIterator for Lexer {
    type Item = Spanned<Token, usize>;
    type IntoIter = Vec<>::IntoIter<Self::Item>;

    fn into_iter(&self) -> Self::IntoIter {
        self.tokens.into_iter()
    }
}*/