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

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_match_single_symbol_token() {
        assert_eq!(match_single_symbol_token('!'), Some(Token::Bang));
        assert_eq!(match_single_symbol_token('}'), Some(Token::RightCurlyBrace));
        assert_eq!(match_single_symbol_token('{'), Some(Token::LeftCurlyBrace));
        // ... test all other single symbols ...
        assert_eq!(match_single_symbol_token(')'), Some(Token::RightParenthesis));
        assert_eq!(match_single_symbol_token('('), Some(Token::LeftParenthesis));
        assert_eq!(match_single_symbol_token('\''), Some(Token::Apostrophe));
        assert_eq!(match_single_symbol_token('\n'), Some(Token::NewLine));
        assert_eq!(match_single_symbol_token('|'), Some(Token::VerticalBar));
        assert_eq!(match_single_symbol_token('@'), Some(Token::At));

        // Test for characters not in the match list
        assert_eq!(match_single_symbol_token('a'), None);
        assert_eq!(match_single_symbol_token('1'), None);
        assert_eq!(match_single_symbol_token(' '), None);
    }

    #[test]
fn test_match_double_symbol_token() {
    assert_eq!(match_double_symbol_token('!', '='), Some(Token::NotEqual));
    assert_eq!(match_double_symbol_token('=', '>'), Some(Token::Send));
    assert_eq!(match_double_symbol_token('=', '='), Some(Token::EqualEqual));
    assert_eq!(match_double_symbol_token('>', '='), Some(Token::GreaterThanEqual));
    assert_eq!(match_double_symbol_token('<', '='), Some(Token::LessThanEqual));

    // Test for character pairs not in the match list
    assert_eq!(match_double_symbol_token('a', 'b'), None);
    assert_eq!(match_double_symbol_token('>', '>'), None);
    assert_eq!(match_double_symbol_token('=', '<'), None);
}

#[test]
fn test_match_tripple_symbol_token() {
    assert_eq!(match_tripple_symbol_token('.', '.', '.'), Some(Token::Ellipsis));
    assert_eq!(match_tripple_symbol_token('<', '=', '>'), Some(Token::Exchange));

    // Test for character triples not in the match list
    assert_eq!(match_tripple_symbol_token('a', 'b', 'c'), None);
    assert_eq!(match_tripple_symbol_token('=', '=', '='), None);
    assert_eq!(match_tripple_symbol_token('<', '<', '<'), None);
}


}