use crate::address_language::AlgorithmParser;
use common::location::Location;
use common::util::read_file;
use lexer::errors::*;
use lexer::lexer::Lexer;
use lexer::token::*;

pub mod ast;

#[allow(clippy::all)]
mod address_language {
    include!(concat!(env!("OUT_DIR"), "/grammar.rs"));
}

pub fn parse(
    source_text: &str,
) -> Result<ast::Algorithm, lalrpop_util::ParseError<Location, TokenKind, LexError>> {
    let lexer = Lexer::new(source_text);
    let ast: Result<ast::Algorithm, lalrpop_util::ParseError<Location, TokenKind, LexError>> =
        AlgorithmParser::new().parse(lexer);
    ast
}

pub fn parse_by_path(
    path: &str,
) -> Result<ast::Algorithm, lalrpop_util::ParseError<Location, TokenKind, LexError>> {
    let file = read_file(path);
    let lexer = Lexer::new(&file);
    let ast: Result<ast::Algorithm, lalrpop_util::ParseError<Location, TokenKind, LexError>> =
        AlgorithmParser::new().parse(lexer);
    ast
}
