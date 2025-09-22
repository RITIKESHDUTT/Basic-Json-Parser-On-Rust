use std::fmt;
use std::fmt::{ Formatter};

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum JsonNumber{
    Integer(i64),
    UnsignedInteger(u64),
    Float(f64)
}

impl fmt::Display for JsonNumber{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self{
            JsonNumber::Integer(i) => write!(f,"{}", i),
            JsonNumber::UnsignedInteger(u) => write!(f, "{}", u),
            JsonNumber::Float(fl) => {
                if fl.is_nan() {
                    write!(f, "null")
                } else if fl.is_infinite(){
                    write!(f, "null")
                } else if fl.fract() == 0.0 && fl.abs() < 1e15{
                    write!(f, "{:.0}", fl)
                }else {
                    write!(f, "{}", fl)
                }
            }
        }
    }
}

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
