use crate::core::{JsonError, JsonNumber};

pub struct NumberParser;

impl NumberParser{
    pub(crate) fn parse(s:&str) -> Result<JsonNumber, JsonError> {
        if s.is_empty() {
            return Err(JsonError::InvalidNumber {line:0, col:0});
        }

        // integer detection without decimal or scientific notation

        if !s.contains('.') && !s.contains('e') && !s.contains('E') {
            //Tries i64 first (most common case)
            if let Ok(i) = s.parse::<i64>() {
                return Ok(JsonNumber::Integer(i));
            }
            // Try u64 for large positive numbers
            if !s.starts_with('-') {
                if let Ok(u) = s.parse::<u64>(){
                    return Ok(JsonNumber::UnsignedInteger(u));
                }
            }
        }
        s.parse::<f64>()
            .map(JsonNumber::Float)
            .map_err(|_| JsonError::InvalidNumber { line: 0, col: 0 })
    }
}

pub struct StringEscaper;

impl StringEscaper {
    pub(crate) fn escape(s: &str) -> String {
        if !Self::needs_escaping(s) {
            return s.to_string();
        }
        let mut result = String::with_capacity(s.len() + s.len() /4);

        for ch in s.chars() {
            match ch {
                '"' => result.push_str("\\\""),
                '\\' => result.push_str("\\\\"),
                '\u{0008}' => result.push_str("\\b"),
                '\u{000C}' =>  result.push_str("\\f"),
                '\n' => result.push_str("\\n"),
                '\r' => result.push_str("\\r"),
                '\t' => result.push_str("\\t"),

                c if c.is_control() && (c as u32) <= 0x1F => {
                    result.push_str(&format!("\\u{:04x}", c as u32));
                }
                '\u{007F}' => result.push_str("\\u007f"),
                '\u{2028}' => result.push_str("\\u2028"),
                '\u{2029}' => result.push_str("\\u2029"),

                c if (c as u32) > 0xFFFF => {
                    let code = c as u32;
                    let high = 0xD800 +({code - 0x10000} >> 10);
                    let low = 0xDC00 + ((code - 0x10000) & 0x3FF);
                    result.push_str(&format!("\\u{:04x}\\u{:04x}", high, low));
                }
                _ => result.push(ch),
            }
        }
        result
    }

    pub(crate) fn unescape_char(c: char) -> Result<char, JsonError> {
        match c {
            '"' => Ok('"'),
            '\\' => Ok('\\'),
            '/' => Ok('/'),
            'b' => Ok('\x08'),
            'f' => Ok('\x0C'),
            'n' => Ok('\n'),
            'r' => Ok('\r'),
            't' => Ok('\t'),
            _ => Err(JsonError::InvalidEscapeSequence {line: 0 , col: 0}),
        }
    }

    pub(crate) fn parse_unicode_hex(hex:&str) -> Result<char, JsonError> {
        if hex.len() != 4 {
            return Err(JsonError::InvalidEscapeSequence {line: 0, col: 0});
        }

        let code_point = u32::from_str_radix(hex, 16)
            .map_err(|_| JsonError::InvalidEscapeSequence {line: 0, col: 0})?;

        char::from_u32(code_point)
            .ok_or(JsonError::InvalidEscapeSequence {line: 0, col: 0})
    }

    fn needs_escaping(s: &str) -> bool {
        s.chars().any(|c| match c{
            '"' | '\\' | '\u{0008}' |  '\u{000C}' | '\n' | '\r' | '\t' => true,
            c if c.is_control() && (c as u32) <= 0x1F => true,
            '\u{007F}' | '\u{2028}' | '\u{2029}' => true,
            c if (c as u32) > 0xFFFF => true,
            _ => false,
        })
    }
}

