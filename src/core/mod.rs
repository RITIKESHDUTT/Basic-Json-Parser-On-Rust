mod json_number;
pub use json_number::JsonNumber;

use std::fmt;
use std::fmt::{ Formatter};

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum Token {
    CurlyLeft,
    CurlyRight,
    SquareLeft,
    SquareRight,
    Colon,
    Comma,
    String(String),
    Number(JsonNumber),
    True,
    False,
    Null,
    Eof,
}

#[derive(Debug, PartialEq, Clone)]
pub enum JsonValue {
    Null,
    Bool(bool),
    Number(JsonNumber),
    String(String),
    Array(Vec<JsonValue>),
    Object(Vec<(String, JsonValue)>),
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum JsonError {
    InvalidToken { line: usize, col: usize },
    UnexpectedEof { line: usize, col: usize }, // Eof - End of file
    InvalidEscapeSequence { line: usize, col: usize },
    InvalidNumber { line: usize, col: usize },
}

impl fmt::Display for JsonError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            JsonError::InvalidToken { line, col } => write!(f, "Invalid token at {}:{}", line, col),
            JsonError::UnexpectedEof { line, col } => write!(f, "Unexpected EOF at {}:{}", line, col),
            JsonError::InvalidEscapeSequence { line, col } => {
                write!(f, "Invalid escape at {}:{}", line, col)
            }
            JsonError::InvalidNumber { line, col } => write!(f, "Invalid number at {}:{}", line, col),
        }
    }
}

impl std::error::Error for JsonError {}