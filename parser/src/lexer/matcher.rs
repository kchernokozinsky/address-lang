use crate::token::*;
pub fn match_single_symbol_token(c: char) -> Option<Token> {
    match c {
        '!' => Some(Token::Bang),
        '}' => Some(Token::RightCurlyBrace),
        '{' => Some(Token::LeftCurlyBrace),
        ']' => Some(Token::RightSquareBracket),
        '[' => Some(Token::LeftSquareBracket),
        ':' => Some(Token::Colon),
        ',' => Some(Token::Comma),
        '/' => Some(Token::Slash),
        '.' => Some(Token::Dot),
        '=' => Some(Token::Equal),
        '>' => Some(Token::GreaterThan),
        '<' => Some(Token::LessThan),
        '%' => Some(Token::Percent),
        '*' => Some(Token::Multiply),
        ')' => Some(Token::RightParenthesis),
        '(' => Some(Token::LeftParenthesis),
        ';' => Some(Token::Semicolon),
        '-' => Some(Token::Minus),
        '+' => Some(Token::Plus),
        '\'' => Some(Token::Apostrophe),
        '\n' => Some(Token::NewLine),
        '|' => Some(Token::VerticalBar),
        '@' => Some(Token::At),
        _ => None,
    }
}

pub fn match_double_symbol_token(a: char, b: char) -> Option<Token> {
    match (a, b) {
        ('!', '=') => Some(Token::NotEqual),
        ('=', '>') => Some(Token::Send),
        ('=', '=') => Some(Token::EqualEqual),
        ('>', '=') => Some(Token::GreaterThanEqual),
        ('<', '=') => Some(Token::LessThanEqual),
        _ => None,
    }
}

pub fn match_tripple_symbol_token(a: char, b: char, c: char) -> Option<Token> {
    match (a, b, c) {
        ('.', '.', '.') => Some(Token::Ellipsis),
        ('<', '=', '>') => Some(Token::Exchange),
        _ => None,
    }
}
