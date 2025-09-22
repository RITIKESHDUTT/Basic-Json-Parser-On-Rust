use std::string::String;
use std::iter::Peekable;
use std::str::Chars;
use crate::core::{JsonError, Token, JsonValue};
use crate::parser::{NumberParser, StringEscaper};

pub struct Lexer<'a>{
    chars: Peekable<Chars<'a>>,
    line: usize,
    col:usize,
}

impl<'a> Lexer<'a> {
    pub(crate) fn new(input: &'a str) -> Self {
        Self {
            chars: input.chars().peekable(),
            line: 1,
            col:1,
        }
    }

    fn advance_char(&mut self) -> Option<char> {
        if let Some(c) = self.chars.next() {
            if c == '\n' {
                self.line +=1;
                self.col = 1;
            } else {
                self.col +=1;
            }
            Some(c)
        } else {
            None
        }
    }

    pub(crate) fn next_token(&mut self) -> Result<Token, JsonError> {
        self.skip_whitespace();

        match self.chars.peek() {
            Some('{') => {self.advance_char(); Ok(Token::CurlyLeft)}
            Some('}') => {self.advance_char(); Ok(Token::CurlyRight)}
            Some('[') => {self.advance_char(); Ok(Token::SquareLeft)}
            Some(']') => {self.advance_char(); Ok(Token::SquareRight)}
            Some(':') => {self.advance_char(); Ok(Token::Colon)}
            Some(',') => {self.advance_char(); Ok(Token::Comma)}
            Some('"') => self.parse_string(),
            Some(c) if c.is_ascii_digit() || *c == '-' => self.parse_number(),
            Some('t') => self.parse_literal("true", Token::True),
            Some('f') => self.parse_literal("false", Token::False),
            Some('n') => self.parse_literal("null", Token::Null),
            Some(_) => {
                let (line, col) = (self.line, self.col);
                self.advance_char();
                Err(JsonError::InvalidToken { line, col })
            }
            _ => Ok(Token::Eof)
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(&c) = self.chars.peek(){
            if c.is_whitespace() {
                self.advance_char();
            } else {
                break;
            }
        }
    }
    fn parse_string(&mut self) -> Result<Token, JsonError> {
        let (start_line, start_col) = (self.line, self.col);
        self.advance_char();

        let mut string = String::new();
        loop{
            match self.advance_char() {
                Some('"') => return Ok(Token::String(string)),
                Some('\\') => {
                    match self.advance_char(){
                        Some('u') => {
                            let mut hex_chars = String::with_capacity(4);
                            for _ in 0..4 {
                                match self.advance_char(){
                                    Some(c) if c.is_ascii_hexdigit() => hex_chars.push(c),
                                    _ => return Err(JsonError::InvalidEscapeSequence {
                                        line: self.line,
                                        col: self.col,
                                    }),
                                }
                            }
                            let ch = StringEscaper::parse_unicode_hex(&hex_chars)
                                .map_err(|_| JsonError::InvalidEscapeSequence {
                                    line:self.line,
                                    col:self.col,
                                })?;
                            string.push(ch);
                        }
                        Some(c) => {
                            let escaped = StringEscaper::unescape_char(c)
                                .map_err(|_| JsonError::InvalidEscapeSequence {
                                    line:self.line,
                                    col:self.col,
                                })?;
                            string.push(escaped);
                        }
                        None => return Err(JsonError::InvalidEscapeSequence {
                            line:self.line,
                            col:self.col,
                        }),
                    }
                }
                Some(c) => string.push(c),
                None => return Err(JsonError::UnexpectedEof{
                    line: start_line,
                    col: start_col,
                }),
            }
        }
    }

    fn parse_number(&mut self) -> Result<Token, JsonError> {
        let (start_line, start_col) = (self.line, self.col);
        let mut number_str = String::new();

        while let Some(&c) = self.chars.peek() {
            match c{
                '0'..='9' | '.' | '-' | '+' | 'e' | 'E' => {
                    number_str.push(c);
                    self.advance_char();
                }
                _ => break,
            }
        }
        NumberParser::parse(&number_str)
            .map(Token::Number)
            .map_err(|_| JsonError::InvalidNumber {
                line:start_line,
                col:start_col,
            })
    }

    fn parse_literal(&mut self, expected: &str, token: Token) -> Result<Token, JsonError>{
        let (start_line, start_col) = (self.line, self.col);
        for expected_char in expected.chars() {
            match self.advance_char(){
                Some(c) if c ==expected_char => continue,
                _=> return Err(JsonError::InvalidToken{
                    line:start_line,
                    col:start_col,
                })
            }
        }
        Ok(token)
    }
}

pub struct Parser <'a> {
    lexer: Lexer<'a>,
    current_token: Token,
}

impl<'a> Parser <'a> {
    pub(crate) fn new(input: &'a str) -> Result<Self, JsonError> {
        let mut lexer = Lexer::new(input);
        let current_token = lexer.next_token()?;
        Ok(Self {lexer, current_token})
    }

    fn advance(&mut self) -> Result<(), JsonError> {
        self.current_token = self.lexer.next_token()?;
        Ok(())
    }

    pub(crate) fn parse(&mut self) -> Result<JsonValue, JsonError>{
        let value = self.parse_value()?;
        if !matches!(self.current_token, Token::Eof) {
            return Err(JsonError::InvalidToken {
                line: self.lexer.line,
                col: self.lexer.col,
            });
        }
        Ok(value)
    }

    fn parse_value(&mut self) -> Result<JsonValue, JsonError> {
        match &self.current_token {
            Token::Null => { self.advance()?; Ok(JsonValue::Null) }
            Token::True => { self.advance()?; Ok(JsonValue::Bool(true)) }
            Token::False => { self.advance()?; Ok(JsonValue::Bool(false)) }
            Token::Number(n) => { let value = n.clone(); self.advance()?; Ok(JsonValue::Number(value)) }
            Token::String(s) => { let value = s.clone(); self.advance()?; Ok(JsonValue::String(value)) }
            Token::SquareLeft => self.parse_array(),
            Token::CurlyLeft => self.parse_object(),
            _ => Err(JsonError::InvalidToken { line: self.lexer.line, col: self.lexer.col }),
        }
    }

    fn parse_array(&mut self) -> Result<JsonValue, JsonError> {
        self.advance()?; // consume '['
        let mut elements = Vec::new();

        if matches!(self.current_token, Token::SquareRight) {
            self.advance()?; // consume ']'
            return Ok(JsonValue::Array(elements));
        }

        loop {
            elements.push(self.parse_value()?);

            match self.current_token {
                Token::Comma => { self.advance()?; }
                Token::SquareRight => { self.advance()?; break; }
                _ => return Err(JsonError::InvalidToken { line: self.lexer.line, col: self.lexer.col }),
            }
        }

        Ok(JsonValue::Array(elements))
    }

    fn parse_object(&mut self) -> Result<JsonValue, JsonError> {
        self.advance()?; // consume '{'
        let mut pairs = Vec::new();

        if matches!(self.current_token, Token::CurlyRight) {
            self.advance()?; // consume '}'
            return Ok(JsonValue::Object(pairs));
        }

        loop {
            let key = match &self.current_token {
                Token::String(s) => { let key = s.clone(); self.advance()?; key }
                _ => return Err(JsonError::InvalidToken { line: self.lexer.line, col: self.lexer.col }),
            };

            if !matches!(self.current_token, Token::Colon) {
                return Err(JsonError::InvalidToken { line: self.lexer.line, col: self.lexer.col });
            }
            self.advance()?; // consume ':'

            let value = self.parse_value()?;
            pairs.push((key, value));

            match self.current_token {
                Token::Comma => { self.advance()?; }
                Token::CurlyRight => { self.advance()?; break; }
                _ => return Err(JsonError::InvalidToken { line: self.lexer.line, col: self.lexer.col }),
            }
        }

        Ok(JsonValue::Object(pairs))
    }
}
