
pub mod matcher;
pub mod token;
pub mod errors;
use errors::*;
use crate::location::*;
use matcher::*;
use queues::*;
use std::iter::Peekable;
use std::str::CharIndices;
use token::*;

pub struct Lexer<'a> {
    input: &'a str,
    char_indices: Peekable<CharIndices<'a>>,
    current_index: usize,
    current_char: Option<(usize, char)>,
    location: Location,
    is_eof: bool,
    skipped_chars: Queue<Option<(usize, char)>>,
}

impl<'a> Lexer<'a> {
    pub fn new(chars: &'a str) -> Self {
        let mut peekable_chars = chars.char_indices().peekable();

        let current = peekable_chars.next();
        let mut location = Location::new(1, 1);
        if let Some((_, '\n')) = current {
            location.newline();
        }

        Lexer {
            input: &chars,
            char_indices: peekable_chars,
            current_index: 0,
            current_char: current,
            location,
            is_eof: false,
            skipped_chars: queue![],
        }
    }

    fn skip_whitespace_and_comments(&mut self) {
        while let Some(c) = self.peek_char() {
            if c == '#' {
                while let Some(c_) = self.peek_char() {
                    if c_ == '\n' {
                        break;
                    }
                    self.next_char();
                }
            } else {
                if c == '\n' || !c.is_ascii_whitespace() {
                    return;
                }
                self.next_char();
            }
        }
    }

    fn peek_char(&mut self) -> Option<char> {
        self.current_char.map(|(_, c)| c)
    }

    fn next_char(&mut self) {
        let v = if self.skipped_chars.size() != 0 {
            match self.skipped_chars.remove() {
                Ok(cur) => cur,
                _ => None,
            }
        } else {
            self.char_indices.next()
        };
        if let Some((i, c)) = v {
            self.current_index = i;
            self.current_char = Some((i, c));

            if c == '\n' {
                self.location.newline();
            } else {
                self.location.go_right();
            }
        } else {
            self.location.go_right();
            self.current_index = self.input.len();
            self.current_char = None;
        }
    }

    fn move_back(&mut self) {
        if self.current_index == 0 {
            return;
        }

        let _ = self.skipped_chars.add(self.current_char.clone());
        // Retrieve the index and character of the previous position

        if let Some((.., prev_char)) = self.current_char.clone() {
            // Adjust the location accordingly
            if prev_char == '\n' {
                self.location.move_back_newline();
            } else {
                self.location.go_left();
            }
        }

        let prev: Option<(usize, char)> =
            self.input[..self.current_index].char_indices().rev().next();
        if let Some((prev_index, prev_char)) = prev {
            // Update the index and current character to the previous position
            self.current_index = prev_index;
            self.current_char = Some((prev_index, prev_char));
        };
    }

    fn loc(&mut self) -> Location {
        self.location
    }

    fn next_keyword_or_identirier_literal(&mut self) -> Span {
        let start = self.current_index;
        let start_loc = self.loc();

        while let Some(c) = self.peek_char() {
            if !c.is_ascii_alphanumeric() && c != '_' {
                break;
            }
            self.next_char();
        }
        let end = self.current_index;
        let end_loc = self.loc();

        let t = match &self.input[start..end] {
            "const" => TokenKind::Const,
            "let" => TokenKind::Let,
            "null" => TokenKind::Null,
            "true" => TokenKind::True,
            "false" => TokenKind::False,
            "del" => TokenKind::Del,
            "L" => TokenKind::Loop,
            "P" => TokenKind::Predicate,
            "R" => TokenKind::Replace,
            "or" => TokenKind::Or,
            "and" => TokenKind::And,
            "SP" => TokenKind::SubProgram,
            "return" => TokenKind::Return,

            s => TokenKind::Identifier(s.to_string()),
        };

        (start_loc, t, end_loc)
    }

    fn determine_number(&mut self) -> Result<Span, LexError> {
        let start = self.current_index;
        let start_loc = self.location;

        // Consume all digits
        while let Some(c) = self.peek_char() {
            if !c.is_ascii_digit() {
                break;
            }
            self.next_char();
        }

        // Check if the number is potentially a float
        if self.peek_char() == Some('.') {
            // If it's a dot, parse as float
            self.next_float(&self.input[start..self.current_index], start_loc)
        } else {
            // Parse as integer
            let int_str = &self.input[start..self.current_index];
            match int_str.parse::<i64>() {
                Ok(value) => Ok((start_loc, TokenKind::IntegerLiteral(value), self.location)),
                Err(_) => Err(LexError::IntegerFormatError(start_loc, int_str.to_string())),
            }
        }
    }

    pub fn next_float(
        &mut self,
        integral_part: &str,
        start_location: Location,
    ) -> Result<Span, LexError> {
        self.next_char(); // Consume the '.'

        let fractional_start = self.current_index;

        while let Some(c) = self.peek_char() {
            if !c.is_ascii_digit() {
                break;
            }
            self.next_char();
        }

        let fractional_part = &self.input[fractional_start..self.current_index];
        let float_str = format!("{}.{}", integral_part, fractional_part);

        match float_str.parse::<f64>() {
            Ok(value) => Ok((start_location, TokenKind::FloatLiteral(value), self.location)),
            Err(_) => Err(LexError::FloatFormatError(self.location, float_str)),
        }
    }

    fn next_quoted_str_literal(&mut self) -> Result<Span, LexError> {
        let start = self.current_index;
        let start_loc = self.loc();

        self.next_char();

        loop {
            let c = if let Some(c) = self.peek_char() {
                c
            } else {
                return Err(LexError::UnterminatedStringLiteral(start_loc));
            };
            self.next_char();
            if c == '\"' {
                break;
            }
        }

        let end = self.current_index;
        let end_loc = self.loc();

        let id = &self.input[(start + 1)..(end - 1)];

        let t = TokenKind::StringLiteral(id.to_string());

        Ok((start_loc, t, end_loc))
    }

    fn next_symbol_token(&mut self, c: char) -> Option<Span> {
        let start_loc = self.loc();

        match match_single_symbol_token(c) {
            Some(initial_t) => {
                self.next_char();
                if let Some(next_char_b) = self.peek_char() {
                    match match_double_symbol_token(c, next_char_b) {
                        Some(_) => self.next_double_symbol_token_(c, next_char_b),
                        None => {
                            self.next_char();
                            if let Some(next_char_c) = self.peek_char() {
                                match match_tripple_symbol_token(c, next_char_b, next_char_c) {
                                    Some(token) => {
                                        self.next_char();
                                        return Some((start_loc, token, self.loc()));
                                    }
                                    None => {
                                        self.move_back();
                                        return Some((start_loc, initial_t, self.loc()));
                                    }
                                }
                            } else {
                                self.move_back();
                                // self.move_back();
                                return Some((start_loc, initial_t, self.loc()));
                            };
                        }
                    }
                } else {
                    Some((start_loc, initial_t, self.loc()))
                }
            }
            None => self.next_double_symbol_token(c),
        }
    }

    fn next_double_symbol_token_(&mut self, a: char, b: char) -> Option<Span> {
        let start_loc = self.loc();
        let t = match_double_symbol_token(a, b);
        match t {
            Some(t) => {
                self.next_char();
                let c: char = if let Some(c) = self.peek_char() {
                    c
                } else {
                    // if out of characters, return a two-character token
                    return Some((start_loc, t, self.loc()));
                };

                match match_tripple_symbol_token(a, b, c) {
                    Some(token) => {
                        // if it is a three-character token but return it and increase the index by 1
                        self.next_char();
                        return Some((start_loc, token, self.loc()));
                    }
                    //otherwise return a two-character token
                    None => return Some((start_loc, t, self.loc())),
                }
            }
            None => {
                let c: char = if let Some(c) = self.peek_char() {
                    c
                } else {
                    // if out of characters, return a None
                    return None;
                };

                match match_tripple_symbol_token(a, b, c) {
                    Some(token) => {
                        // if it is a three-character token but return it and increase the index by 1
                        self.next_char();
                        return Some((start_loc, token, self.loc()));
                    }
                    // otherwise return a two-character token
                    None => return None,
                }
            }
        }
    }

    fn next_double_symbol_token(&mut self, a: char) -> Option<Span> {
        let start_loc = self.loc();
        self.next_char();

        let b = self.peek_char()?;
        let t = match_double_symbol_token(a, b);
        match t {
            Some(t) => {
                self.next_char();
                let c: char = if let Some(c) = self.peek_char() {
                    c
                } else {
                    // if out of characters, return a two-character token
                    return Some((start_loc, t, self.loc()));
                };

                match match_tripple_symbol_token(a, b, c) {
                    Some(token) => {
                        // if it is a three-character token, return it and increase the index by 1
                        let end_location = self.loc();
                        self.next_char();
                        return Some((start_loc, token, end_location));
                    }
                    // otherwise return a two-character token
                    None => return Some((start_loc, t, self.loc())),
                }
            }
            None => {
                let c: char = if let Some(c) = self.peek_char() {
                    c
                } else {
                    // if out of characters, return a two-character token
                    return None;
                };

                match match_tripple_symbol_token(a, b, c) {
                    Some(token) => {
                        // if it is a three-character token but return it and increase the index by 1
                        self.next_char();
                        return Some((start_loc, token, self.loc()));
                    }
                    // otherwise return a two-character token
                    None => return None,
                }
            }
        }
    }
}

pub type Span = (Location, TokenKind, Location);

impl<'a> Iterator for Lexer<'a> {
    type Item = Result<Span, LexError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.skip_whitespace_and_comments();

        // Return None if no characters left, handling the end of a
        let c = match self.peek_char() {
            None => {
                if !self.is_eof {
                    self.is_eof = true;
                    self.loc().go_right();
                    return Some(Ok((self.loc(), TokenKind::EndOfFile, self.loc())));
                } else {
                    return None;
                }
            }
            Some(c) => c,
        };

        // Processing next token based on the current character
        Some(if c.is_ascii_alphabetic() {
            Ok(self.next_keyword_or_identirier_literal())
        } else if c.is_ascii_digit() {
            self.determine_number()
        } else if c == '"' {
            self.next_quoted_str_literal()
        } else if c == '\n' {
            self.next_char();
            Ok((self.loc(), TokenKind::NewLine, self.loc()))
        } else {
            // Process symbol or return an error
            self.next_symbol_token(c)
                .map_or_else(|| Err(LexError::Unexpected(self.loc(), c)), Ok)
        })
    }
}

// --- TESTS ---

#[cfg(test)]
mod tests {

    use super::*;

    fn assert_lexer_output(
        lexer: &mut Lexer,
        expected_start_loc: Location,
        expected_token: TokenKind,
        expected_end_loc: Location,
    ) {
        match lexer.next() {
            Some(Ok((start_loc, token, end_loc))) => {
                assert_eq!(start_loc, expected_start_loc);
                assert_eq!(token, expected_token);
                assert_eq!(end_loc, expected_end_loc);
            }
            _ => panic!("Expected a valid token, but got an error or None"),
        }
    }

    #[test]
    fn test_tokenize_keywords() {
        let keywords = [
            ("const", TokenKind::Const, 1, 6),
            ("let", TokenKind::Let, 1, 4),
            ("null", TokenKind::Null, 1, 5),
            ("true", TokenKind::True, 1, 5),
            ("false", TokenKind::False, 1, 6),
            ("del", TokenKind::Del, 1, 4),
            ("L", TokenKind::Loop, 1, 2),
            ("P", TokenKind::Predicate, 1, 2),
            ("R", TokenKind::Replace, 1, 2),
            ("or", TokenKind::Or, 1, 3),
            ("and", TokenKind::And, 1, 4),
        ];

        for (keyword, expected_token, row, col) in keywords {
            let mut lexer = Lexer::new(keyword);
            let expected_start_loc = Location::new(1, 1);
            let expected_end_loc = Location::new(row, col);

            assert_lexer_output(
                &mut lexer,
                expected_start_loc,
                expected_token,
                expected_end_loc,
            );
        }
    }

    #[test]
    fn test_tokenize_identifier() {
        let mut lexer = Lexer::new("myVariable");
        let expected_start_loc = Location::new(1, 1);
        let expected_end_loc = Location::new(1, 11); // Adjust according to the length of "myVariable"
        let expected_token = TokenKind::Identifier("myVariable".to_string());

        assert_lexer_output(
            &mut lexer,
            expected_start_loc,
            expected_token,
            expected_end_loc,
        );
    }

    #[test]
    fn test_tokenize_integer() {
        let mut lexer = Lexer::new("12345");
        let expected_start_loc = Location::new(1, 1);
        let expected_end_loc = Location::new(1, 6); // Adjust according to the length of "12345"
        let expected_token = TokenKind::IntegerLiteral(12345);

        assert_lexer_output(
            &mut lexer,
            expected_start_loc,
            expected_token,
            expected_end_loc,
        );
    }

    #[test]
    fn test_tokenize_float() {
        let mut lexer = Lexer::new("123.45");
        let expected_start_loc = Location::new(1, 1);
        let expected_end_loc = Location::new(1, 7); // Adjust according to the length of "123.45"
        let expected_token = TokenKind::FloatLiteral(123.45);

        assert_lexer_output(
            &mut lexer,
            expected_start_loc,
            expected_token,
            expected_end_loc,
        );
    }

    #[test]
    fn test_tokenize_string() {
        let mut lexer = Lexer::new("\"Hello, World!\"");
        let expected_start_loc = Location::new(1, 1);
        let expected_end_loc = Location::new(1, 16); // Adjust according to the length of the string
        let expected_token = TokenKind::StringLiteral("Hello, World!".to_string());

        assert_lexer_output(
            &mut lexer,
            expected_start_loc,
            expected_token,
            expected_end_loc,
        );
    }

    #[test]
    fn test_tokenize_symbols() {
        let mut lexer = Lexer::new("+");
        let expected_start_loc = Location::new(1, 1);
        let expected_end_loc = Location::new(1, 2); // Symbols are usually single characters
        let expected_token = TokenKind::Plus; // Replace with the actual token for '+'

        assert_lexer_output(
            &mut lexer,
            expected_start_loc,
            expected_token,
            expected_end_loc,
        );
        // Repeat for other symbols like '-', '*', '/', etc.
    }

    #[test]
    fn test_ignore_comments() {
        let mut lexer = Lexer::new("# This is a comment\nx");
        lexer.next();
        let expected_start_loc = Location::new(2, 1); // 'x' starts at the second line
        let expected_end_loc = Location::new(2, 2);
        let expected_token = TokenKind::Identifier("x".to_string());

        assert_lexer_output(
            &mut lexer,
            expected_start_loc,
            expected_token,
            expected_end_loc,
        );
    }

    #[test]
    fn test_error_unterminated_string() {
        let mut lexer = Lexer::new("\"unterminated string");
        match lexer.next() {
            Some(Err(LexError::UnterminatedStringLiteral(loc))) => {
                assert_eq!(loc, Location::new(1, 1)); // The location where the error occurs
            }
            _ => panic!("Expected an unterminated string literal error"),
        }
    }
}
