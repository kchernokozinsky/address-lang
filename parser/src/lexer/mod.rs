// Copyright 2022 Sean Kelleher. All rights reserved.
// Use of this source code is governed by an MIT
// licence that can be found in the LICENCE file.
pub mod location;
pub mod matcher;
pub mod token;
use location::*;
use matcher::*;
use queues::*;
use std::iter::Peekable;
use std::str::CharIndices;
use token::*;

#[derive(Debug)]
pub enum LexError {
    Unexpected(Location, char),
    UnterminatedStringLiteral(Location),
    FloatFormatError(Location, String),
    IntegerFormatError(Location, String),
}

pub struct Lexer<'a> {
    input: &'a str,
    char_indices: Peekable<CharIndices<'a>>,
    current_index: usize,
    current_char: Option<(usize, char)>,
    location: Location,
    has_ended: bool,
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
            has_ended: false,
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
            self.current_index = self.input.len();
            self.current_char = None;
        }
    }

    fn move_back(&mut self) {
        if self.current_index == 0 {
            return;
        }

        self.skipped_chars.add(self.current_char.clone());
        // Retrieve the index and character of the previous position
        let prev = self.input[..self.current_index].char_indices().rev().next();

        if let Some((prev_index, prev_char)) = prev {
            // Update the index and current character to the previous position
            self.current_index = prev_index;
            self.current_char = Some((prev_index, prev_char));

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
            "const" => Token::Const,
            "let" => Token::Let,
            "null" => Token::Null,
            "true" => Token::True,
            "false" => Token::False,
            "del" => Token::Del,
            "L" => Token::Loop,
            "P" => Token::Predicate,
            "R" => Token::Replace,
            "or" => Token::Or,
            "and" => Token::And,

            s => Token::Identifier(s.to_string()),
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
            self.next_float(&self.input[start..self.current_index])
        } else {
            // Parse as integer
            let int_str = &self.input[start..self.current_index];
            match int_str.parse::<i64>() {
                Ok(value) => Ok((start_loc, Token::IntegerLiteral(value), self.location)),
                Err(_) => Err(LexError::IntegerFormatError(start_loc, int_str.to_string())),
            }
        }
    }

    pub fn next_float(&mut self, integral_part: &str) -> Result<Span, LexError> {
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
            Ok(value) => Ok((self.location, Token::FloatLiteral(value), self.location)),
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
                return Err(LexError::UnterminatedStringLiteral(self.location));
            };
            self.next_char();
            if c == '\"' {
                break;
            }
        }

        let end = self.current_index;
        let end_loc = self.loc();

        let id = &self.input[(start + 1)..(end - 1)];

        let t = Token::StringLiteral(id.to_string());

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

pub type Span = (Location, Token, Location);

impl<'a> Iterator for Lexer<'a> {
    type Item = Result<Span, LexError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.skip_whitespace_and_comments();

        // Return None if no characters left, handling the end of a
        let c = match self.peek_char() {
            None => {
                if !self.has_ended {
                    self.has_ended = true;
                    return Some(Ok((self.loc(), Token::EndOfFile, self.loc())));
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
        } else {
            // Process symbol or return an error
            self.next_symbol_token(c)
                .map_or_else(|| Err(LexError::Unexpected(self.loc(), c)), Ok)
        })
    }
}
