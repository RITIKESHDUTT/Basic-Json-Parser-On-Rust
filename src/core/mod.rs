mod json_number;
mod json_value;

pub use json_number::JsonNumber;
pub use self::json_value::JsonValue;

use std::fmt;
use std::fmt::Formatter;
// In core/mod.rs or a new limits.rs
pub const MAX_STRING_LENGTH: usize = 10_000_000;  // 10MB
pub const MAX_DEPTH: usize = 128;
pub const MAX_ARRAY_LENGTH: usize = 100_000;
pub const MAX_OBJECT_KEYS: usize = 100_000;
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
pub enum JsonError {
    InvalidToken { line: usize, col: usize },
    UnexpectedEof { line: usize, col: usize }, // Eof - End of file
    InvalidEscapeSequence { line: usize, col: usize },
    InvalidNumber { line: usize, col: usize },
    
    MaxDepthExceeded { max: usize },
    MaxStringSizeExceeded { max: usize },
    MaxArraySizeExceeded { max: usize },
    MaxObjectSizeExceeded { max: usize },
}
#[derive(Debug, Clone)]
pub struct ParserLimits {
    pub max_depth: usize,
    pub max_string_length: usize,
    pub max_array_length: usize,
    pub max_object_keys: usize,
}
impl Default for ParserLimits {
    fn default() -> Self {
        Self {
            max_depth: MAX_DEPTH,           // Prevent stack overflow
            max_string_length: MAX_STRING_LENGTH,  // 10MB strings
            max_array_length: MAX_ARRAY_LENGTH,      // 100k elements
            max_object_keys: MAX_OBJECT_KEYS,       // 100k keys
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
            JsonError::MaxDepthExceeded {max} => write!(f, "Maximum DepthExceeded Limit-  {}", max),
            JsonError::MaxStringSizeExceeded {max} => write!(f, "Maximum StringsizeExceeded Limit- {}",max),
            JsonError::MaxArraySizeExceeded {max} => write!(f, "Maximum ArraySizeExceeded Limit - {}", max),
            JsonError::MaxObjectSizeExceeded {max} => write!(f, "Maximum ObjectSizeExceeded Limit- {}", max),
           
        }
    }
}

impl std::error::Error for JsonError {}