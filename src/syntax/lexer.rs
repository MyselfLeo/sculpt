//! Custom lexer passed to LALRPOP to parse the commands.
//! The need of a custom lexer comes from the fact that some tokens may have different semantics
//! depending on the state of the engine, notably identifiers which needs to have their type
//! defined (terms, propositions, etc).
//! Note that this parser does not parse REPL-specific commands (exit, help, etc). Those are managed
//! by the REPL itself.

use std::fmt::{Debug, Display, Formatter};
use std::str::CharIndices;

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


pub enum LexicalError {
    UnknownToken(Spanned<String, usize>)
}

impl Debug for LexicalError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            LexicalError::UnknownToken(s) => write!(f, "Unknown token '{}'", s.1)
        }
    }
}

pub type Spanned<Token, Loc> = (Loc, Token, Loc);


#[derive(Debug, Clone)]
pub enum Token {
    Def,
    Thm,
    Admit,
    Qed,
    Use,

    Ident(String),

    Falsum,
    Exists,
    Forall,
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

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Token::Def => "Def",
            Token::Thm => "Thm",
            Token::Admit => "Admit",
            Token::Qed => "Qed",
            Token::Use => "Use",
            Token::Ident(s) => s,
            Token::Falsum => "falsum",
            Token::Exists => "exists",
            Token::Forall => "forall",
            Token::Wave => "~",
            Token::DoubleArrow => "=>",
            Token::Or => "\\/",
            Token::And => "/\\",
            Token::OpenParen => "(",
            Token::CloseParen => ")",
            Token::Comma => ",",
            Token::Dot => ".",
            Token::DoubleColon => "::",
        };
        write!(f, "{str}")
    }
}


#[derive(Debug)]
enum BufState {
    AlphaNum,
    Sym,
    Idle
}

pub struct Lexer<'input> {
    //pub tokens: Vec<Spanned<Token, usize>>,

    iterator: CharIndices<'input>,
    buf_state: BufState,
    buf: String,
    buf_start: usize,
    curr_pos: usize,
    line_skip: bool
}

impl<'input> Lexer<'input> {
    pub fn from(s: &'input str) -> Self {
        Self {
            iterator: s.char_indices(),
            buf_state: BufState::Idle,
            buf: String::new(),
            buf_start: 0,
            curr_pos: 0,
            line_skip: false
        }
    }


    pub fn next_token(&mut self) -> Result<Option<Token>, LexicalError> {
        match self.next() {
            None => Ok(None),
            Some(r) => match r {
                Ok((_, t, _)) => Ok(Some(t)),
                Err(r) => Err(r)
            }
        }
    }

    pub fn is_finished(&self) -> bool {
        let mut peekable = self.iterator.clone().peekable();
        match peekable.peek() {
            Some(_) => false,
            None => true
        }
    }


    fn consume_buf(&mut self) -> Result<Spanned<Token, usize>, LexicalError> {
        match Self::token_from_str(&self.buf, &self.buf_state) {
            None => {
                Err(LexicalError::UnknownToken((self.buf_start, self.buf.clone(), self.curr_pos - 1)))
            },
            Some(t) => {
                let res = (self.buf_start, t, self.curr_pos - 1);
                self.buf.clear();
                self.buf_start = self.curr_pos;
                self.buf_state = BufState::Idle;
                Ok(res)
            }
        }
    }




    fn token_from_str(buf: &String, buf_state: &BufState) -> Option<Token> {
        let res = match buf_state {
            BufState::Idle => unreachable!(),
            BufState::AlphaNum => {
                match buf.as_str() {
                    "Def" => Token::Def,
                    "Thm" => Token::Thm,
                    "Admit" => Token::Admit,
                    "Qed" => Token::Qed,
                    "Use" => Token::Use,

                    "falsum" => Token::Falsum,
                    "exists" => Token::Exists,
                    "forall" => Token::Forall,
                    _ => Token::Ident(buf.clone())
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
                    _ => return None
                }
            }
        };

        Some(res)
    }
}




impl<'input> Iterator for Lexer<'input> {
    type Item = Result<Spanned<Token, usize>, LexicalError>;

    fn next(&mut self) -> Option<Self::Item> {
        'char_iter:
        while let Some((i, c)) = self.iterator.next() {
            let mut to_be_yield = None;

            self.curr_pos = i;

            if c == '\n' {
                self.line_skip = false;
                continue
            }

            if self.line_skip { continue }

            // When the start of a comment is encountered, clean current buf and return early
            if self.buf == COMMENT_START {
                self.buf.clear();
                self.buf_state = BufState::Idle;
                self.line_skip = true;
                continue 'char_iter;
            }

            if SYMBOLS.contains(&self.buf.as_str()) {
                let res = self.consume_buf();
                self.buf_state = BufState::Idle;
                to_be_yield = Some(res);
            }

            if c.is_whitespace() {
                if !self.buf.is_empty() {
                    to_be_yield = Some(self.consume_buf())
                };
            }

            // buf state is used to allow alphanumeric tokens (idents) and symbolic tokens
            // (operators) to be written with no space between them
            else {
                if is_ident_allowed(c) {
                    match self.buf_state {
                        BufState::AlphaNum => (),
                        BufState::Idle => {
                            self.buf_state = BufState::AlphaNum;
                            self.buf_start = i;
                        },
                        BufState::Sym => {
                            let tok = self.consume_buf();
                            self.buf_state = BufState::AlphaNum;
                            to_be_yield = Some(tok);
                        }
                    }
                }
                else {
                    match self.buf_state {
                        BufState::Sym => (),
                        BufState::Idle => {
                            self.buf_start = i;
                            self.buf_state = BufState::Sym;
                        },
                        BufState::AlphaNum => {
                            let tok = self.consume_buf();
                            self.buf_state = BufState::Sym;
                            to_be_yield = Some(tok);
                        }
                    }
                }

                self.buf.push(c);
            };

            if to_be_yield.is_some() {
                return to_be_yield;
            }
        }

        if !self.buf.is_empty() {
            self.curr_pos += 1;
            Some(self.consume_buf())
        } else {
            None
        }
    }
}