#[allow(dead_code)]
#[allow(unused_imports)]

use std::fmt;
use std::fmt::{ Formatter};

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum JsonError {
    InvalidToken { line: usize, col: usize },
    UnexpectedEof { line: usize, col: usize }, // Eof - End of file
    InvalidEscapeSequence { line: usize, col: usize },
    InvalidNumber { line: usize, col: usize },
}

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
impl JsonNumber {
    pub(crate) fn from_str_simple(s:&str) -> Result<Self, std::num::ParseFloatError> {
        if !s.contains('.') && !s.contains('e') && !s.contains('E') {
            if let Ok(i) = s.parse::<i64>() {
                return Ok(JsonNumber::Integer(i));
            }
            if !s.starts_with('-') {
                if let Ok(u) = s.parse::<u64>() {
                    return Ok(JsonNumber::UnsignedInteger(u));
                }
            }
        }
        s.parse::<f64>().map(JsonNumber::Float)
    }
    pub(crate) fn as_f64(&self) -> f64 {
        match self {
            JsonNumber::Integer(i) => *i as f64,
            JsonNumber::UnsignedInteger(u) => *u as f64,
            JsonNumber::Float(f) => *f,
        }
    }
    pub fn as_i64(&self) -> Option<i64> {
        match self {
            JsonNumber::Integer(i) => Some(*i),
            JsonNumber::UnsignedInteger(u) if *u <= i64::MAX as u64 => Some(*u as i64),
            JsonNumber::Float(f) => {
                // Check if it's a whole number within i64 range
                if f.is_finite() && f.fract() == 0.0 {
                    let i = *f as i64;
                    // Verify no precision was lost in the conversion
                    if i as f64 == *f {
                        Some(i)
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            _ => None
        }
    }
    pub(crate) fn is_integer(&self) -> bool {
        match self {
            JsonNumber::Integer(_) | JsonNumber::UnsignedInteger(_) => true,
            JsonNumber::Float(fl) => fl.fract() ==0.0,
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

impl JsonValue {
    pub(crate) fn to_json_string(&self) -> String {
            self.to_json_string_internal(0, false)
        }


    pub(crate) fn to_json_string_indent(&self, indent: usize) -> String {
        self.to_json_string_internal(indent, true)
    }

    fn to_json_string_internal(&self, indent: usize, pretty: bool) -> String {
        let ind = "    ".repeat(indent);
        let next_indent = indent + 1;
        match self {
            JsonValue::Null => "null".to_string(),
            JsonValue::Bool(b) => b.to_string(),
            JsonValue::Number(n) => n.to_string(),
            JsonValue::String(s) => format!("\"{}\"", s),
            JsonValue::Array(arr) => {
                if arr.is_empty() { "[]".to_string() }
                else {
                    let items: Vec<String> = arr
                        .iter()
                        .map(|v| {
                            if pretty {
                                format!("{}{}", "    ".repeat(next_indent), v.to_json_string_internal(next_indent, true))
                            } else {
                                v.to_json_string_internal(next_indent, false)
                            }
                        })
                        .collect();
                    if pretty {
                        format!("[\n{}\n{}]", items.join(",\n"), ind)
                    } else {
                        format!("[{}]", items.join(","))
                    }
                }
            }
            JsonValue::Object(obj) => {
                if obj.is_empty() { "{}".to_string() }
                else {
                    let items: Vec<String> = obj
                        .iter()
                        .map(|(k, v)| {
                            if pretty {
                                format!("{}\"{}\": {}", "    ".repeat(next_indent), k, v.to_json_string_internal(next_indent, true))
                            } else {
                                format!("\"{}\":{}", k, v.to_json_string_internal(next_indent, false))
                            }
                        })
                        .collect();
                    if pretty {
                        format!("{{\n{}\n{}}}", items.join(",\n"), ind)
                    } else {
                        format!("{{{}}}", items.join(","))
                    }
                }
            }
        }
    }
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
