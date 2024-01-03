// Copyright 2022 Sean Kelleher. All rights reserved.
// Use of this source code is governed by an MIT
// licence that can be found in the LICENCE file.
pub mod location;
pub mod token;
use location::*;
use std::iter::Peekable;
use std::str::CharIndices;
use token::*;

#[derive(Debug)]
pub enum LexError {
    Unexpected(Location, char),
}

pub struct Lexer<'input> {
    raw_chars: &'input str,
    chars: Peekable<CharIndices<'input>>,
    index: usize,
    current: Option<(usize, char)>,
    location: Location,
    end: bool,
    skipped_chars: Vec<Option<(usize, char)>>,
}

impl<'input> Lexer<'input> {
    pub fn new(chars: &'input str) -> Self {
        let mut peekable_chars = chars.char_indices().peekable();

        let current = peekable_chars.next();
        let mut location = Location::new(1, 1);
        if let Some((_, '\n')) = current {
            location.newline();
        }

        Lexer {
            raw_chars: &chars,
            chars: peekable_chars,
            index: 0,
            current,
            location,
            end: false,
            skipped_chars: vec![],
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
        if let Some((_, c)) = self.current {
            Some(c)
        } else {
            None
        }
    }

    fn next_char(&mut self) {
        let v = if !self.skipped_chars.is_empty() {
            match self.skipped_chars.pop() {
                Some(cur) => cur,
                None => None,
            }
        } else {
            self.chars.next()
        };
        if let Some((i, c)) = v {
            self.index = i;
            self.current = Some((i, c));

            if c == '\n' {
                self.location.newline();
            } else {
                self.location.go_right();
            }
        } else {
            self.index = self.raw_chars.len();
            self.current = None;
        }
    }

    fn move_back(&mut self) {
        if self.index == 0 {
            return;
        }

        self.skipped_chars.push(self.current.clone());
        // Retrieve the index and character of the previous position
        let prev = self.raw_chars[..self.index].char_indices().rev().next();

        if let Some((prev_index, prev_char)) = prev {
            // Update the index and current character to the previous position
            self.index = prev_index;
            self.current = Some((prev_index, prev_char));

            // Adjust the location accordingly
            if prev_char == '\n' {
                self.location.move_back_newline();
            } else {
                self.location.go_left();
            }
        }
    }

    fn loc(&mut self) -> Location {
        self.location
    }

    fn next_keyword_or_identirier_literal(&mut self) -> Span {
        let start = self.index;
        let start_loc = self.loc();

        while let Some(c) = self.peek_char() {
            if !c.is_ascii_alphanumeric() && c != '_' {
                break;
            }
            self.next_char();
        }
        let end = self.index;
        let end_loc = self.loc();

        let t = match &self.raw_chars[start..end] {
            "const" => Token::CONST,
            "let" => Token::LET,
            "null" => Token::NULL,
            "true" => Token::TRUE,
            "false" => Token::FALSE,
            "del" => Token::DEL,
            "L" => Token::LOOP,
            "P" => Token::PREDICATE,
            "or" => Token::OR,
            "and" => Token::AND,

            s => Token::IDENTIFIER(s.to_string()),
        };

        (start_loc, t, end_loc)
    }

    fn next_int(&mut self) -> Span {
        let start = self.index;
        let start_loc = self.loc();

        while let Some(c) = self.peek_char() {
            if !c.is_ascii_digit() {
                break;
            }
            self.next_char();
        }
        let end = self.index;
        let end_loc = self.loc();

        let raw_int = &self.raw_chars[start..end];
        let value: i64 = raw_int.parse().unwrap();
        let t = Token::INTEGER_LITERAL(value);

        (start_loc, t, end_loc)
    }

    fn next_quoted_str_literal(&mut self) -> Span {
        let start = self.index;
        let start_loc = self.loc();

        self.next_char();

        while let Some(c) = self.peek_char() {
            self.next_char();
            if c == '\"' {
                break;
            }
        }
        let end = self.index;
        let end_loc = self.loc();

        let id = &self.raw_chars[(start + 1)..(end - 1)];

        let t = Token::STRING_LITERAL(id.to_string());

        (start_loc, t, end_loc)
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
                    // если закончились символы то вернуть двух символьный токен
                    return Some((start_loc, t, self.loc()));
                };

                match match_tripple_symbol_token(a, b, c) {
                    Some(token) => {
                        // если это трехсимвольный токен но вернуть его и увеличить индекс на 1
                        self.next_char();
                        return Some((start_loc, token, self.loc()));
                    }
                    // иначе вернуть двух символьный токен
                    None => return Some((start_loc, t, self.loc())),
                }
            }
            None => {
                let c: char = if let Some(c) = self.peek_char() {
                    c
                } else {
                    // если закончились символы то вернуть двух символьный токен
                    return None;
                };

                match match_tripple_symbol_token(a, b, c) {
                    Some(token) => {
                        // если это трехсимвольный токен но вернуть его и увеличить индекс на 1
                        self.next_char();
                        return Some((start_loc, token, self.loc()));
                    }
                    // иначе вернуть двух символьный токен
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
                    // если закончились символы то вернуть двух символьный токен
                    return Some((start_loc, t, self.loc()));
                };

                match match_tripple_symbol_token(a, b, c) {
                    Some(token) => {
                        // если это трехсимвольный токен но вернуть его и увеличить индекс на 1
                        let end_location = self.loc();
                        self.next_char();
                        return Some((start_loc, token, end_location));
                    }
                    // иначе вернуть двух символьный токен
                    None => return Some((start_loc, t, self.loc())),
                }
            }
            None => {
                let c: char = if let Some(c) = self.peek_char() {
                    c
                } else {
                    // если закончились символы то вернуть двух символьный токен
                    return None;
                };

                match match_tripple_symbol_token(a, b, c) {
                    Some(token) => {
                        // если это трехсимвольный токен но вернуть его и увеличить индекс на 1
                        self.next_char();
                        return Some((start_loc, token, self.loc()));
                    }
                    // иначе вернуть двух символьный токен
                    None => return None,
                }
            }
        }
    }

}

pub type Span = (Location, Token, Location);

impl<'input> Iterator for Lexer<'input> {
    type Item = Result<Span, LexError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.skip_whitespace_and_comments();

        let c = self.peek_char();

        if let Some(c) = c {
            let result = if c.is_ascii_alphabetic() {
                Ok(self.next_keyword_or_identirier_literal())
            } else if c.is_ascii_digit() {
                Ok(self.next_int())
            } else if c == '"' {
                Ok(self.next_quoted_str_literal())
            } else {
                if let Some(t) = self.next_symbol_token(c) {
                    Ok(t)
                } else {
                    Err(LexError::Unexpected(self.loc(), c))
                }
            };
            Some(result)
        } else {
            if !self.end {
                self.end = true;
                Some(Ok((self.loc(), Token::END_OF_FILE, self.loc())))
            } else {
                None
            }
        }
    }
}

fn match_single_symbol_token(c: char) -> Option<Token> {
    match c {
        '!' => Some(Token::BANG),
        '}' => Some(Token::RIGHT_CURLY_BRACE),
        '{' => Some(Token::LEFT_CURLY_BRACE),
        ']' => Some(Token::RIGHT_SQUARE_BRACKET),
        '[' => Some(Token::LEFT_SQUARE_BRACKET),
        ':' => Some(Token::COLON),
        ',' => Some(Token::COMMA),
        '/' => Some(Token::OPERATOR_SLASH),
        '.' => Some(Token::OPERATOR_DOT),
        '=' => Some(Token::OPERATOR_EQUAL),
        '>' => Some(Token::OPERATOR_GREATER_THAN),
        '<' => Some(Token::OPERATOR_LESS_THAN),
        '%' => Some(Token::OPERATOR_PERCENT),
        '*' => Some(Token::OPERATOR_MULTIPLY),
        ')' => Some(Token::RIGHT_PARENTHESIS),
        '(' => Some(Token::LEFT_PARENTHESIS),
        ';' => Some(Token::SEMICOLON),
        '-' => Some(Token::OPERATOR_MINUS),
        '+' => Some(Token::OPERATOR_PLUS),
        '\'' => Some(Token::OPERATOR_APOSTROPHE),
        '\n' => Some(Token::NEW_LINE),

        _ => None,
    }
}

fn match_double_symbol_token(a: char, b: char) -> Option<Token> {
    match (a, b) {
        ('!', '=') => Some(Token::OPERATOR_NOT_EQUAL),
        ('=', '>') => Some(Token::OPERATOR_RIGHT_ARROW),
        ('=', '=') => Some(Token::OPERATOR_EQUAL_EQUAL),
        ('>', '=') => Some(Token::OPERATOR_GREATER_THAN_EQUAL),
        ('<', '=') => Some(Token::OPERATOR_LESS_THAN_EQUAL),

        _ => None,
    }
}

fn match_tripple_symbol_token(a: char, b: char, c: char) -> Option<Token> {
    match (a, b, c) {
        ('.', '.', '.') => Some(Token::OPERATOR_ELLIPSIS),
        ('<', '=', '>') => Some(Token::OPERATOR_REPLACE),
        _ => None,
    }
}
