use std::iter::Peekable;
use std::str::Chars;
use crate::core::{JsonError, Token, JsonValue};

pub struct Lexer<'a> {
    chars: Peekable<Chars<'a>>,
    line: usize,
    col: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer { chars: input.chars().peekable(), line: 1, col: 1 }
    }

    fn advance_char(&mut self) -> Option<char> {
        let next = self.chars.next();
        if let Some(c) = next {
            if c == '\n' { self.line += 1; self.col = 1; } else { self.col += 1; }
        }
        next
    }

    pub fn next_token(&mut self) -> Result<Token, JsonError> {
        self.skip_whitespace();
        match self.chars.peek().copied() {
            Some('{') => { self.advance_char(); Ok(Token::CurlyLeft) },
            Some('}') => { self.advance_char(); Ok(Token::CurlyRight) },
            Some('[') => { self.advance_char(); Ok(Token::SquareLeft) },
            Some(']') => { self.advance_char(); Ok(Token::SquareRight) },
            Some(':') => { self.advance_char(); Ok(Token::Colon) },
            Some(',') => { self.advance_char(); Ok(Token::Comma) },
            Some('"') => self.parse_string(),
            Some('-') | Some('0'..='9') => self.parse_number(),
            Some('t') => self.parse_literal("true", Token::True),
            Some('f') => self.parse_literal("false", Token::False),
            Some('n') => self.parse_literal("null", Token::Null),
            Some(_) => { let (l, c) = (self.line, self.col); self.advance_char(); Err(JsonError::InvalidToken{line:l,col:c}) },
            None => Ok(Token::Eof),
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(&c) = self.chars.peek() {
            if c.is_whitespace() { self.advance_char(); } else { break; }
        }
    }

    fn parse_string(&mut self) -> Result<Token, JsonError> {
        let (start_line, start_col) = (self.line, self.col);
        self.advance_char();
        let mut string = String::new();

        loop {
            let c = match self.advance_char() {
                Some(ch) => ch,
                None => return Err(JsonError::UnexpectedEof{line:start_line,col:start_col}),
            };
            match c {
                '"' => return Ok(Token::String(string)),
                '\\' => {
                    let esc = self.advance_char().ok_or(JsonError::InvalidEscapeSequence{line:self.line,col:self.col})?;
                    string.push(match esc { '"' | '\\' | '/' => esc, 'b'=> '\x08', 'f'=> '\x0C', 'n'=> '\n', 'r'=> '\r', 't'=> '\t', _ => return Err(JsonError::InvalidEscapeSequence{line:self.line,col:self.col}) });
                },
                _ => string.push(c),
            }
        }
    }

    fn parse_number(&mut self) -> Result<Token, JsonError> {
        let (start_line, start_col) = (self.line, self.col);
        let mut s = String::new();
        while let Some(&c) = self.chars.peek() {
            if c.is_ascii_digit() || c == '.' || c == '-' || c == 'e' || c == 'E' { s.push(c); self.advance_char(); } else { break; }
        }
        if s.is_empty() { return Err(JsonError::InvalidNumber{line:start_line,col:start_col}); }
        let n = s.parse::<f64>().map_err(|_| JsonError::InvalidNumber{line:start_line,col:start_col})?;
        Ok(Token::Number(n))
    }

    fn parse_literal(&mut self, lit: &str, tok: Token) -> Result<Token, JsonError> {
        let (start_line, start_col) = (self.line, self.col);
        for expected in lit.chars() {
            if Some(expected) != self.advance_char() { return Err(JsonError::InvalidToken{line:start_line,col:start_col}); }
        }
        Ok(tok)
    }
}

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current_token: Result<Token, JsonError>,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut lexer = Lexer::new(input);
        let current_token = lexer.next_token();
        Parser { lexer, current_token }
    }

    fn advance(&mut self) -> Result<(), JsonError> {
        self.current_token = self.lexer.next_token();
        self.current_token.clone().map(|_| ())
    }

    pub fn parse(&mut self) -> Result<JsonValue, JsonError> {
        let value = self.parse_value()?;
        // Ensure EOF
        match &self.current_token {
            Ok(Token::Eof) => Ok(value),
            _ => Err(JsonError::InvalidToken {
                line: self.lexer.line,
                col: self.lexer.col,
            }),
        }
    }

    fn parse_value(&mut self) -> Result<JsonValue, JsonError> {
        match self.current_token.clone()? {
            Token::Null => { self.advance()?; Ok(JsonValue::Null) },
            Token::True => { self.advance()?; Ok(JsonValue::Bool(true)) },
            Token::False => { self.advance()?; Ok(JsonValue::Bool(false)) },
            Token::Number(n) => { self.advance()?; Ok(JsonValue::Number(n)) },
            Token::String(s) => { self.advance()?; Ok(JsonValue::String(s)) },
            Token::SquareLeft => self.parse_array(),
            Token::CurlyLeft => self.parse_object(),
            _ => Err(JsonError::InvalidToken {
                line: self.lexer.line,
                col: self.lexer.col,
            }),
        }
    }

    fn parse_array(&mut self) -> Result<JsonValue, JsonError> {
        self.advance()?; // skip '['
        let mut arr = Vec::new();

        if let Ok(Token::SquareRight) = &self.current_token {
            self.advance()?; // skip ']'
            return Ok(JsonValue::Array(arr));
        }

        loop {
            arr.push(self.parse_value()?);

            match &self.current_token {
                Ok(Token::Comma) => { self.advance()?; },
                Ok(Token::SquareRight) => { self.advance()?; break; },
                _ => return Err(JsonError::InvalidToken {
                    line: self.lexer.line,
                    col: self.lexer.col,
                }),
            }
        }

        Ok(JsonValue::Array(arr))
    }

    fn parse_object(&mut self) -> Result<JsonValue, JsonError> {
        self.advance()?; // skip '{'
        let mut obj = Vec::new();

        if let Ok(Token::CurlyRight) = &self.current_token {
            self.advance()?; // skip '}'
            return Ok(JsonValue::Object(obj));
        }

        loop {
            let key = match self.current_token.clone()? {
                Token::String(s) => { self.advance()?; s },
                _ => return Err(JsonError::InvalidToken {
                    line: self.lexer.line,
                    col: self.lexer.col,
                }),
            };

            if let Ok(Token::Colon) = &self.current_token {
                self.advance()?; // skip ':'
            } else {
                return Err(JsonError::InvalidToken {
                    line: self.lexer.line,
                    col: self.lexer.col,
                });
            }

            let val = self.parse_value()?;
            obj.push((key, val));

            match &self.current_token {
                Ok(Token::Comma) => { self.advance()?; },
                Ok(Token::CurlyRight) => { self.advance()?; break; },
                _ => return Err(JsonError::InvalidToken {
                    line: self.lexer.line,
                    col: self.lexer.col,
                }),
            }
        }

        Ok(JsonValue::Object(obj))
    }
}