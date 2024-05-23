use errors::LexError;
use lexer::Lexer;

pub mod errors;
pub mod lexer;
mod matcher;
pub mod token;

pub fn tokenize(
    str: &str,
) -> Vec<
    Result<
        (
            common::location::Location,
            token::TokenKind,
            common::location::Location,
        ),
        LexError,
    >,
> {
    let mut tokens = vec![];
    let lexer = Lexer::new(str);
    for item in lexer {
        tokens.push(item);
    }
    tokens
}
