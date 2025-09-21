use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum JsonError {
    InvalidToken { line: usize, col: usize },
    UnexpectedEof { line: usize, col: usize },
    InvalidEscapeSequence { line: usize, col: usize },
    InvalidNumber { line: usize, col: usize },
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    CurlyLeft,
    CurlyRight,
    SquareLeft,
    SquareRight,
    Colon,
    Comma,
    String(String),
    Number(f64),
    True,
    False,
    Null,
    Eof,
}

#[derive(Debug, PartialEq, Clone)]
pub enum JsonValue {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<JsonValue>),
    Object(Vec<(String, JsonValue)>),
}

impl JsonValue {
    pub(crate) fn to_json_string(&self) -> String {
        match self {
            JsonValue::Null => "null".to_string(),
            JsonValue::Bool(b) => b.to_string(),
            JsonValue::Number(n) => n.to_string(),
            JsonValue::String(s) => format!("{:?}", s),
            JsonValue::Array(arr) => {
                let contents: Vec<String> = arr.iter().map(|v| v.to_json_string()).collect();
                format!("[{}]", contents.join(","))
            }
            JsonValue::Object(obj) => {
                let contents: Vec<String> = obj
                    .iter()
                    .map(|(k, v)| format!("{:?}:{}", k, v.to_json_string()))
                    .collect();
                format!("{{{}}}", contents.join(","))
            }
        }
    }
}

impl fmt::Display for JsonError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
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
