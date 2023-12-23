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
const DEFAULT_SYMBOLS: [&str; 7] = [
    "~",
    "=>",
    "\\/",
    "/\\",
    "(",
    ")",
    ","
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
    Symbol(String),     // Non-alphanumeric symbol
    Dot,                // .
    DoubleComma         // ::
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


pub struct Lexer {
    pub tokens: Vec<Spanned<Token, usize>>
}

impl Lexer {
    pub fn lex(input: &str, context: &Context) -> Result<Self, ParserError> {
        enum BufState {
            AlphaNum,
            Sym,
            Idle
        }

        let mut buf_state = BufState::Idle;
        let mut buf = String::new();
        let mut start: usize = 0;


        let mut tokens: Vec<Spanned<Token, usize>> = vec![];


        let mut push_buf = |buf: &str, start: usize, end: usize, vec: &mut Vec<_>| {
            let s = if buf == "::" { (start, Token::DoubleComma, end) }
                else if DEFAULT_KEYWORDS.contains(&buf) { (start, Token::Keyword(buf.to_string()), end) }
                else if DEFAULT_RULES.contains(&buf) { (start, Token::RuleName(buf.to_string()), end) }
                else if DEFAULT_SYMBOLS.contains(&buf) { (start, Token::Symbol(buf.to_string()), end) }
                else if context.terms.contains_key(buf) { (start, Token::Term(buf.to_string()), end) }
                else if context.relations.contains_key(buf) { (start, Token::Relation(buf.to_string()), end) }
                else { (start, Token::Ident(buf.to_string()), end) };

            vec.push(s);
        };


        'char_iter: for (i, c) in input.char_indices() {
            // When the start of a comment is encountered, clean current buf and return early
            if buf == COMMENT_START {
                break 'char_iter;
            }



            /*// end buffer early if starting with a symbol
            // allow to separate '/\foo' as '/\' & 'foo'
            if DEFAULT_SYMBOLS.contains(&buf.as_str()) {
                tokens.push(Ok((start, Token::Symbol(buf.clone()), i-1)));

                buf.clear();
                start = i+1;
                continue 'char_iter;
            }*/

            if c == '.' {
                if !buf.is_empty() {
                    push_buf(&buf, start, i-1, &mut tokens);
                    buf.clear();
                }
                tokens.push((i, Token::Dot, i));
                start = i+1;
                buf_state = BufState::Idle;
                continue 'char_iter;
            }

            if c.is_whitespace() {
                if !buf.is_empty() {
                    push_buf(&buf, start, i-1, &mut tokens);
                    buf.clear();
                }
                start = i+1;
                buf_state = BufState::Idle;
                continue 'char_iter;
            }

            // buf state is used to allow alphanumeric tokens (idents) and symbolic tokens
            // (operators) to be written with no space between them
            if is_ident_allowed(c) {
                match buf_state {
                    BufState::Idle | BufState::AlphaNum => (),
                    BufState::Sym => {
                        push_buf(&buf, start, i-1, &mut tokens);
                        start = i+1;
                    }
                }
                buf_state = BufState::AlphaNum;
            }
            else {
                match buf_state {
                    BufState::Idle | BufState::Sym => (),
                    BufState::AlphaNum => {
                        push_buf(&buf, start, i-1, &mut tokens);
                        start = i+1;
                    }
                }
                buf_state = BufState::Sym;
            }


            buf.push(c);
            start = i;
        }

        push_buf(&buf, start, input.len()-1, &mut tokens)?;

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