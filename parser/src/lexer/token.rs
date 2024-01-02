//! Different token definitions.
use std::fmt::{self};

/// Tokens representing basic elements of the address language syntax.
#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    IDENTIFIER (String),
    INTEGER_LITERAL (i64),
    FLOAT_LITERAL (f64),
    STRING_LITERAL (String),
    NEW_LINE,
    END_OF_FILE,
    LEFT_PARENTHESIS,
    RIGHT_PARENTHESIS,
    LEFT_SQUARE_BRACKET,
    RIGHT_SQUARE_BRACKET,
    COLON,
    COMMA,
    SEMICOLON,
    OPERATOR_MULTIPLY,
    OPERATOR_PLUS,
    OPERATOR_MINUS,
    OPERATOR_SLASH,
    OPERATOR_VERTICAL_BAR, // '|'
    OPERATOR_AMPERSAND,    // '&'
    OPERATOR_LESS_THAN,
    OPERATOR_GREATER_THAN,
    OPERATOR_EQUAL,
    OPERATOR_DOT,
    OPERATOR_PERCENT,
    LEFT_CURLY_BRACE,
    RIGHT_CURLY_BRACE,
    OPERATOR_EQUAL_EQUAL,
    OPERATOR_NOT_EQUAL,
    OPERATOR_LESS_THAN_EQUAL,
    OPERATOR_GREATER_THAN_EQUAL,
    OPERATOR_LEFT_SHIFT,
    OPERATOR_RIGHT_SHIFT,
    OPERATOR_DOUBLE_SLASH, // '//'
    OPERATOR_RIGHT_ARROW,
    OPERATOR_ELLIPSIS,
    OPERATOR_APOSTROPHE,
    OPERATOR_REPLACE,
    LOOP,
    PREDICATE,
    BANG,

    // Basic Keywords:
    FALSE,
    NULL,
    TRUE,
    AND,
    DEL,
    NOT,
    OR,
    LET,
    CONST,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Token::*;

        match self {
            IDENTIFIER (value) => write!(f, "{}", value),
            INTEGER_LITERAL (value) => write!(f, "{}", value),
            FLOAT_LITERAL (value) => write!(f, "{}", value),
            STRING_LITERAL (value) => write!(f, "\"{}\"", value),
            NEW_LINE => f.write_str("\\n"),
            END_OF_FILE => f.write_str("EOF"),
            LEFT_PARENTHESIS => f.write_str("("),
            RIGHT_PARENTHESIS => f.write_str(")"),
            LEFT_SQUARE_BRACKET => f.write_str("["),
            RIGHT_SQUARE_BRACKET => f.write_str("]"),
            COLON => f.write_str(":"),
            COMMA => f.write_str(","),
            SEMICOLON => f.write_str(";"),
            OPERATOR_MULTIPLY => f.write_str("*"),
            OPERATOR_PLUS => f.write_str("+"),
            OPERATOR_MINUS => f.write_str("-"),
            OPERATOR_SLASH => f.write_str("/"),
            OPERATOR_VERTICAL_BAR => f.write_str("|"),
            OPERATOR_AMPERSAND => f.write_str("&"),
            OPERATOR_LESS_THAN => f.write_str("<"),
            OPERATOR_GREATER_THAN => f.write_str(">"),
            OPERATOR_EQUAL => f.write_str("="),
            OPERATOR_DOT => f.write_str("."),
            OPERATOR_PERCENT => f.write_str("%"),
            LEFT_CURLY_BRACE => f.write_str("{"),
            RIGHT_CURLY_BRACE => f.write_str("}"),
            OPERATOR_EQUAL_EQUAL => f.write_str("=="),
            OPERATOR_NOT_EQUAL => f.write_str("!="),
            OPERATOR_LESS_THAN_EQUAL => f.write_str("<="),
            OPERATOR_GREATER_THAN_EQUAL => f.write_str(">="),
            OPERATOR_LEFT_SHIFT => f.write_str("<<"),
            OPERATOR_RIGHT_SHIFT => f.write_str(">>"),
            OPERATOR_DOUBLE_SLASH => f.write_str("//"),
            OPERATOR_RIGHT_ARROW => f.write_str("=>"),
            OPERATOR_REPLACE => f.write_str("<=>"),
            OPERATOR_ELLIPSIS => f.write_str("..."),
            OPERATOR_APOSTROPHE => f.write_str("'"),
            BANG => f.write_str("!"),

            // Basic Keywords:
            FALSE => f.write_str("false"),
            NULL => f.write_str("null"),
            TRUE => f.write_str("true"),
            AND => f.write_str("and"),
            DEL => f.write_str("del"),
            LOOP => f.write_str("L"),
            PREDICATE => f.write_str("P"), //
            NOT => f.write_str("not"),
            OR => f.write_str("or"),
            LET => f.write_str("let"),
            CONST => f.write_str("const"),
        }
    }
}
