use lexer::lexer::Lexer;
use util::read_file;
use crate::address_language::AlgorithmParser;

pub mod ast;
pub mod evaluation;
pub mod util;
pub mod typings;

#[allow(clippy::all)]
mod address_language {
    include!(concat!(env!("OUT_DIR"), "/grammar.rs"));
}

pub fn parse(
    source_text: &str,
) -> Result<ast::Algorithm, lalrpop_util::ParseError<lexer::location::Location, lexer::token::TokenKind, lexer::errors::LexError>> {
    let lexer = Lexer::new(source_text);
    let ast: Result<ast::Algorithm, lalrpop_util::ParseError<lexer::location::Location, lexer::token::TokenKind, lexer::errors::LexError>>  = AlgorithmParser::new().parse(lexer);
    ast
}

pub fn parse_by_path(
    path: &str,
) -> Result<ast::Algorithm, lalrpop_util::ParseError<lexer::location::Location, lexer::token::TokenKind, lexer::errors::LexError>> {
    let file = read_file(path);
    let lexer = Lexer::new(&file);
    let ast: Result<ast::Algorithm, lalrpop_util::ParseError<lexer::location::Location, lexer::token::TokenKind, lexer::errors::LexError>>  = AlgorithmParser::new().parse(lexer);
    ast
}