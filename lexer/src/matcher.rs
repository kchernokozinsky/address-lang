use crate::token::*;
pub fn match_single_symbol_token(c: char) -> Option<TokenKind> {
    match c {
        '!' => Some(TokenKind::Bang),
        '}' => Some(TokenKind::RightCurlyBrace),
        '{' => Some(TokenKind::LeftCurlyBrace),
        ']' => Some(TokenKind::RightSquareBracket),
        '[' => Some(TokenKind::LeftSquareBracket),
        ':' => Some(TokenKind::Colon),
        ',' => Some(TokenKind::Comma),
        '/' => Some(TokenKind::Slash),
        '.' => Some(TokenKind::Dot),
        '=' => Some(TokenKind::Equal),
        '>' => Some(TokenKind::GreaterThan),
        '<' => Some(TokenKind::LessThan),
        '%' => Some(TokenKind::Percent),
        '*' => Some(TokenKind::Multiply),
        ')' => Some(TokenKind::RightParenthesis),
        '(' => Some(TokenKind::LeftParenthesis),
        ';' => Some(TokenKind::Semicolon),
        '-' => Some(TokenKind::Minus),
        '+' => Some(TokenKind::Plus),
        '\'' => Some(TokenKind::Apostrophe),
        '|' => Some(TokenKind::VerticalBar),
        '@' => Some(TokenKind::At),
        _ => None,
    }
}

pub fn match_double_symbol_token(a: char, b: char) -> Option<TokenKind> {
    match (a, b) {
        ('!', '=') => Some(TokenKind::NotEqual),
        ('=', '>') => Some(TokenKind::Send),
        ('=', '=') => Some(TokenKind::EqualEqual),
        ('>', '=') => Some(TokenKind::GreaterThanEqual),
        ('<', '=') => Some(TokenKind::LessThanEqual),
        (':', ':') => Some(TokenKind::ColonColon),
        _ => None,
    }
}

pub fn match_tripple_symbol_token(a: char, b: char, c: char) -> Option<TokenKind> {
    match (a, b, c) {
        ('.', '.', '.') => Some(TokenKind::Ellipsis),
        ('<', '=', '>') => Some(TokenKind::Exchange),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match_single_symbol_token() {
        assert_eq!(match_single_symbol_token('!'), Some(TokenKind::Bang));
        assert_eq!(
            match_single_symbol_token('}'),
            Some(TokenKind::RightCurlyBrace)
        );
        assert_eq!(
            match_single_symbol_token('{'),
            Some(TokenKind::LeftCurlyBrace)
        );
        // ... test all other single symbols ...
        assert_eq!(
            match_single_symbol_token(')'),
            Some(TokenKind::RightParenthesis)
        );
        assert_eq!(
            match_single_symbol_token('('),
            Some(TokenKind::LeftParenthesis)
        );
        assert_eq!(match_single_symbol_token('\''), Some(TokenKind::Apostrophe));
        assert_eq!(match_single_symbol_token('|'), Some(TokenKind::VerticalBar));
        assert_eq!(match_single_symbol_token('@'), Some(TokenKind::At));

        // Test for characters not in the match list
        assert_eq!(match_single_symbol_token('a'), None);
        assert_eq!(match_single_symbol_token('1'), None);
        assert_eq!(match_single_symbol_token(' '), None);
    }

    #[test]
    fn test_match_double_symbol_token() {
        assert_eq!(
            match_double_symbol_token('!', '='),
            Some(TokenKind::NotEqual)
        );
        assert_eq!(match_double_symbol_token('=', '>'), Some(TokenKind::Send));
        assert_eq!(
            match_double_symbol_token('=', '='),
            Some(TokenKind::EqualEqual)
        );
        assert_eq!(
            match_double_symbol_token('>', '='),
            Some(TokenKind::GreaterThanEqual)
        );
        assert_eq!(
            match_double_symbol_token('<', '='),
            Some(TokenKind::LessThanEqual)
        );
        assert_eq!(
            match_double_symbol_token(':', ':'),
            Some(TokenKind::ColonColon)
        );

        // Test for character pairs not in the match list
        assert_eq!(match_double_symbol_token('a', 'b'), None);
        assert_eq!(match_double_symbol_token('>', '>'), None);
        assert_eq!(match_double_symbol_token('=', '<'), None);
    }

    #[test]
    fn test_match_tripple_symbol_token() {
        assert_eq!(
            match_tripple_symbol_token('.', '.', '.'),
            Some(TokenKind::Ellipsis)
        );
        assert_eq!(
            match_tripple_symbol_token('<', '=', '>'),
            Some(TokenKind::Exchange)
        );

        // Test for character triples not in the match list
        assert_eq!(match_tripple_symbol_token('a', 'b', 'c'), None);
        assert_eq!(match_tripple_symbol_token('=', '=', '='), None);
        assert_eq!(match_tripple_symbol_token('<', '<', '<'), None);
    }
}
