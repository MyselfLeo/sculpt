pub mod lexer;

use lalrpop_util::lalrpop_mod;
lalrpop_mod!(pub parser, "/syntax/parser.rs");