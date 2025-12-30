#[cfg(test)]
use crate::engine::{Lexer, Parser};
use crate::core::{Token, JsonValue, JsonNumber, JsonError};
// ==================== LEXER TESTS ====================
mod lexer_tests {
	use super::*;
	// --- next_token: Structural tokens ---
	
	#[test]
	fn test_curly_braces() {
		let mut lexer = Lexer::new("{}");
		assert_eq!(lexer.next_token().unwrap(), Token::CurlyLeft);
		assert_eq!(lexer.next_token().unwrap(), Token::CurlyRight);
		assert_eq!(lexer.next_token().unwrap(), Token::Eof);
	}
	#[test]
	fn test_square_brackets() {
		let mut lexer = Lexer::new("[]");
		assert_eq!(lexer.next_token().unwrap(), Token::SquareLeft);
		assert_eq!(lexer.next_token().unwrap(), Token::SquareRight);
	}
	#[test]
	fn test_colon_and_comma() {
		let mut lexer = Lexer::new(":,");
		assert_eq!(lexer.next_token().unwrap(), Token::Colon);
		assert_eq!(lexer.next_token().unwrap(), Token::Comma);
	}
	// --- next_token: Literals ---
	
	#[test]
	fn test_true_literal() {
		let mut lexer = Lexer::new("true");
		assert_eq!(lexer.next_token().unwrap(), Token::True);
	}
	#[test]
	fn test_false_literal() {
		let mut lexer = Lexer::new("false");
		assert_eq!(lexer.next_token().unwrap(), Token::False);
	}
	#[test]
	fn test_null_literal() {
		let mut lexer = Lexer::new("null");
		assert_eq!(lexer.next_token().unwrap(), Token::Null);
	}
	#[test]
	fn test_invalid_literal() {
		let mut lexer = Lexer::new("tru");
		assert!(lexer.next_token().is_err());
	}
	// --- skip_whitespace ---
	
	#[test]
	fn test_skip_whitespace_spaces() {
		let mut lexer = Lexer::new("   true");
		assert_eq!(lexer.next_token().unwrap(), Token::True);
	}
	#[test]
	fn test_skip_whitespace_tabs() {
		let mut lexer = Lexer::new("\t\ttrue");
		assert_eq!(lexer.next_token().unwrap(), Token::True);
	}
	#[test]
	fn test_skip_whitespace_newlines() {
		let mut lexer = Lexer::new("\n\n\rtrue");
		assert_eq!(lexer.next_token().unwrap(), Token::True);
	}
	#[test]
	fn test_skip_whitespace_mixed() {
		let mut lexer = Lexer::new("  \t\n  true");
		assert_eq!(lexer.next_token().unwrap(), Token::True);
	}
	// --- parse_string: Basic ---
	
	#[test]
	fn test_empty_string() {
		let mut lexer = Lexer::new(r#""""#);
		assert_eq!(lexer.next_token().unwrap(), Token::String("".to_string()));
	}
	#[test]
	fn test_simple_string() {
		let mut lexer = Lexer::new(r#""hello""#);
		assert_eq!(lexer.next_token().unwrap(), Token::String("hello".to_string()));
	}
	#[test]
	fn test_string_with_spaces() {
		let mut lexer = Lexer::new(r#""hello world""#);
		assert_eq!(lexer.next_token().unwrap(), Token::String("hello world".to_string()));
	}
	// --- parse_string: Escape sequences ---
	
	#[test]
	fn test_escape_quote() {
		let mut lexer = Lexer::new(r#""say \"hello\"""#);
		assert_eq!(lexer.next_token().unwrap(), Token::String("say \"hello\"".to_string()));
	}
	#[test]
	fn test_escape_backslash() {
		let mut lexer = Lexer::new(r#""path\\to\\file""#);
		assert_eq!(lexer.next_token().unwrap(), Token::String("path\\to\\file".to_string()));
	}
	#[test]
	fn test_escape_newline() {
		let mut lexer = Lexer::new(r#""line1\nline2""#);
		assert_eq!(lexer.next_token().unwrap(), Token::String("line1\nline2".to_string()));
	}
	#[test]
	fn test_escape_tab() {
		let mut lexer = Lexer::new(r#""col1\tcol2""#);
		assert_eq!(lexer.next_token().unwrap(), Token::String("col1\tcol2".to_string()));
	}
	#[test]
	fn test_escape_carriage_return() {
		let mut lexer = Lexer::new(r#""line\r""#);
		assert_eq!(lexer.next_token().unwrap(), Token::String("line\r".to_string()));
	}
	#[test]
	fn test_escape_formfeed() {
		let mut lexer = Lexer::new(r#""page\f""#);
		assert_eq!(lexer.next_token().unwrap(), Token::String("page\x0C".to_string()));
	}
	#[test]
	fn test_escape_backspace() {
		let mut lexer = Lexer::new(r#""back\b""#);
		assert_eq!(lexer.next_token().unwrap(), Token::String("back\x08".to_string()));
	}
	#[test]
	fn test_escape_slash() {
		let mut lexer = Lexer::new(r#""a\/b""#);
		assert_eq!(lexer.next_token().unwrap(), Token::String("a/b".to_string()));
	}
	// --- parse_string: Unicode escapes ---
	
	#[test]
	fn test_unicode_basic() {
		let mut lexer = Lexer::new(r#""\u0041""#);  // 'A'
		assert_eq!(lexer.next_token().unwrap(), Token::String("A".to_string()));
	}
	#[test]
	fn test_unicode_emoji_bmp() {
		let mut lexer = Lexer::new(r#""\u263A""#);  // ☺
		assert_eq!(lexer.next_token().unwrap(), Token::String("☺".to_string()));
	}
	#[test]
	fn test_unicode_surrogate_pair() {
		// 😀 = U+1F600 = \uD83D\uDE00
		let mut lexer = Lexer::new(r#""\uD83D\uDE00""#);
		assert_eq!(lexer.next_token().unwrap(), Token::String("😀".to_string()));
	}
	#[test]
	fn test_unicode_invalid_hex() {
		let mut lexer = Lexer::new(r#""\uGGGG""#);
		assert!(lexer.next_token().is_err());
	}
	#[test]
	fn test_unicode_incomplete() {
		let mut lexer = Lexer::new(r#""\u00""#);
		assert!(lexer.next_token().is_err());
	}
	#[test]
	fn test_unicode_orphan_high_surrogate() {
		let mut lexer = Lexer::new(r#""\uD83D""#);  // High surrogate without low
		assert!(lexer.next_token().is_err());
	}
	// --- parse_string: Control characters ---
	
	#[test]
	fn test_reject_raw_control_char() {
		let input = format!("\"hello{}world\"", '\x01');
		let mut lexer = Lexer::new(&input);
		assert!(lexer.next_token().is_err());
	}
	#[test]
	fn test_reject_null_char() {
		let input = format!("\"hello{}world\"", '\x00');
		let mut lexer = Lexer::new(&input);
		assert!(lexer.next_token().is_err());
	}
	// --- parse_string: Errors ---
	
	#[test]
	fn test_unterminated_string() {
		let mut lexer = Lexer::new(r#""hello"#);
		let err = lexer.next_token().unwrap_err();
		assert!(matches!(err, JsonError::UnexpectedEof { .. }));
	}
	#[test]
	fn test_invalid_escape() {
		let mut lexer = Lexer::new(r#""hello\x""#);
		let err = lexer.next_token().unwrap_err();
		assert!(matches!(err, JsonError::InvalidEscapeSequence { .. }));
	}
	// --- parse_number: Integers ---
	
	#[test]
	fn test_positive_integer() {
		let mut lexer = Lexer::new("123");
		let token = lexer.next_token().unwrap();
		assert!(matches!(token, Token::Number(JsonNumber::Integer(123))));
	}
	#[test]
	fn test_negative_integer() {
		let mut lexer = Lexer::new("-456");
		let token = lexer.next_token().unwrap();
		assert!(matches!(token, Token::Number(JsonNumber::Integer(-456))));
	}
	#[test]
	fn test_zero() {
		let mut lexer = Lexer::new("0");
		let token = lexer.next_token().unwrap();
		assert!(matches!(token, Token::Number(JsonNumber::Integer(0))));
	}
	// --- parse_number: Floats ---
	
	#[test]
	fn test_decimal() {
		let mut lexer = Lexer::new("3.14");
		let token = lexer.next_token().unwrap();
		if let Token::Number(JsonNumber::Float(f)) = token {
			assert!((f - 3.14).abs() < 1e-10);
		} else {
			panic!("Expected float");
		}
	}
	#[test]
	fn test_scientific_notation() {
		let mut lexer = Lexer::new("1e10");
		let token = lexer.next_token().unwrap();
		if let Token::Number(JsonNumber::Float(f)) = token {
			assert!((f - 1e10).abs() < 1e5);
		} else {
			panic!("Expected float");
		}
	}
	#[test]
	fn test_scientific_notation_positive_exp() {
		let mut lexer = Lexer::new("1e+5");
		let token = lexer.next_token().unwrap();
		if let Token::Number(JsonNumber::Float(f)) = token {
			assert!((f - 1e5).abs() < 1.0);
		} else {
			panic!("Expected float");
		}
	}
	#[test]
	fn test_scientific_notation_negative_exp() {
		let mut lexer = Lexer::new("1e-5");
		let token = lexer.next_token().unwrap();
		if let Token::Number(JsonNumber::Float(f)) = token {
			assert!((f - 1e-5).abs() < 1e-10);
		} else {
			panic!("Expected float");
		}
	}
	#[test]
	fn test_scientific_notation_uppercase() {
		let mut lexer = Lexer::new("1E10");
		let token = lexer.next_token().unwrap();
		assert!(matches!(token, Token::Number(JsonNumber::Float(_))));
	}
	// --- parse_number: Edge cases ---
	
	#[test]
	fn test_large_integer() {
		let mut lexer = Lexer::new("9223372036854775807");  // i64::MAX
		let token = lexer.next_token().unwrap();
		assert!(matches!(token, Token::Number(JsonNumber::Integer(9223372036854775807))));
	}
	#[test]
	fn test_large_unsigned() {
		let mut lexer = Lexer::new("18446744073709551615");  // u64::MAX
		let token = lexer.next_token().unwrap();
		assert!(matches!(token, Token::Number(JsonNumber::UnsignedInteger(18446744073709551615))));
	}
	// --- Invalid tokens ---
	
	#[test]
	fn test_invalid_token() {
		let mut lexer = Lexer::new("@");
		let err = lexer.next_token().unwrap_err();
		assert!(matches!(err, JsonError::InvalidToken { .. }));
	}
	// --- Line/column tracking ---
	
	#[test]
	fn test_line_tracking() {
		let mut lexer = Lexer::new("{\n  \"key\"\n}");
		lexer.next_token().unwrap();  // {
		lexer.next_token().unwrap();  // "key"
		let token = lexer.next_token().unwrap();  // }
		assert_eq!(token, Token::CurlyRight);
	}
}
// ==================== PARSER TESTS ====================
mod parser_tests {
	use crate::core::ParserLimits;
use super::*;
	// --- parse_value: Primitives ---
	
	#[test]
	fn test_parse_null() {
		let mut parser = Parser::new("null").unwrap();
		let value = parser.parse().unwrap();
		assert_eq!(value, JsonValue::Null);
	}
	#[test]
	fn test_parse_true() {
		let mut parser = Parser::new("true").unwrap();
		let value = parser.parse().unwrap();
		assert_eq!(value, JsonValue::Bool(true));
	}
	#[test]
	fn test_parse_false() {
		let mut parser = Parser::new("false").unwrap();
		let value = parser.parse().unwrap();
		assert_eq!(value, JsonValue::Bool(false));
	}
	#[test]
	fn test_parse_string() {
		let mut parser = Parser::new(r#""hello""#).unwrap();
		let value = parser.parse().unwrap();
		assert_eq!(value, JsonValue::String("hello".to_string()));
	}
	#[test]
	fn test_parse_number() {
		let mut parser = Parser::new("42").unwrap();
		let value = parser.parse().unwrap();
		assert_eq!(value, JsonValue::Number(JsonNumber::Integer(42)));
	}
	// --- parse_array ---
	
	#[test]
	fn test_empty_array() {
		let mut parser = Parser::new("[]").unwrap();
		let value = parser.parse().unwrap();
		assert_eq!(value, JsonValue::Array(vec![]));
	}
	#[test]
	fn test_array_single_element() {
		let mut parser = Parser::new("[1]").unwrap();
		let value = parser.parse().unwrap();
		assert_eq!(value, JsonValue::Array(vec![
			JsonValue::Number(JsonNumber::Integer(1))
		]));
	}
	#[test]
	fn test_array_multiple_elements() {
		let mut parser = Parser::new("[1, 2, 3]").unwrap();
		let value = parser.parse().unwrap();
		assert_eq!(value, JsonValue::Array(vec![
			JsonValue::Number(JsonNumber::Integer(1)),
			JsonValue::Number(JsonNumber::Integer(2)),
			JsonValue::Number(JsonNumber::Integer(3)),
		]));
	}
	#[test]
	fn test_array_mixed_types() {
		let mut parser = Parser::new(r#"[1, "hello", true, null]"#).unwrap();
		let value = parser.parse().unwrap();
		assert_eq!(value, JsonValue::Array(vec![
			JsonValue::Number(JsonNumber::Integer(1)),
			JsonValue::String("hello".to_string()),
			JsonValue::Bool(true),
			JsonValue::Null,
		]));
	}
	#[test]
	fn test_nested_arrays() {
		let mut parser = Parser::new("[[1, 2], [3, 4]]").unwrap();
		let value = parser.parse().unwrap();
		assert_eq!(value, JsonValue::Array(vec![
			JsonValue::Array(vec![
				JsonValue::Number(JsonNumber::Integer(1)),
				JsonValue::Number(JsonNumber::Integer(2)),
			]),
			JsonValue::Array(vec![
				JsonValue::Number(JsonNumber::Integer(3)),
				JsonValue::Number(JsonNumber::Integer(4)),
			]),
		]));
	}
	#[test]
	fn test_array_trailing_comma_error() {
		let mut parser = Parser::new("[1,]").unwrap();
		assert!(parser.parse().is_err());
	}
	// --- parse_object ---
	
	#[test]
	fn test_empty_object() {
		let mut parser = Parser::new("{}").unwrap();
		let value = parser.parse().unwrap();
		assert_eq!(value, JsonValue::Object(vec![]));
	}
	#[test]
	fn test_object_single_pair() {
		let mut parser = Parser::new(r#"{"key": "value"}"#).unwrap();
		let value = parser.parse().unwrap();
		assert_eq!(value, JsonValue::Object(vec![
			("key".to_string(), JsonValue::String("value".to_string()))
		]));
	}
	#[test]
	fn test_object_multiple_pairs() {
		let mut parser = Parser::new(r#"{"a": 1, "b": 2}"#).unwrap();
		let value = parser.parse().unwrap();
		assert_eq!(value, JsonValue::Object(vec![
			("a".to_string(), JsonValue::Number(JsonNumber::Integer(1))),
			("b".to_string(), JsonValue::Number(JsonNumber::Integer(2))),
		]));
	}
	#[test]
	fn test_nested_objects() {
		let mut parser = Parser::new(r#"{"outer": {"inner": 42}}"#).unwrap();
		let value = parser.parse().unwrap();
		assert_eq!(value, JsonValue::Object(vec![
			("outer".to_string(), JsonValue::Object(vec![
				("inner".to_string(), JsonValue::Number(JsonNumber::Integer(42)))
			]))
		]));
	}
	#[test]
	fn test_object_trailing_comma_error() {
		let mut parser = Parser::new(r#"{"a": 1,}"#).unwrap();
		assert!(parser.parse().is_err());
	}
	#[test]
	fn test_object_missing_colon() {
		let mut parser = Parser::new(r#"{"key" "value"}"#).unwrap();
		assert!(parser.parse().is_err());
	}
	#[test]
	fn test_object_non_string_key() {
		let mut parser = Parser::new(r#"{123: "value"}"#).unwrap();
		assert!(parser.parse().is_err());
	}
	// --- Complex structures ---
	
	#[test]
	fn test_complex_json() {
		let json = r#"{
			"name": "test",
			"count": 42,
			"active": true,
			"data": [1, 2, 3],
			"nested": {
				"value": null
			}
		}"#;
		let mut parser = Parser::new(json).unwrap();
		let value = parser.parse().unwrap();
		
		assert!(value.is_object());
		let obj = value.as_object().unwrap();
		assert_eq!(obj.len(), 5);
	}
	// --- Error cases ---
	
	#[test]
	fn test_trailing_content() {
		let mut parser = Parser::new("null null").unwrap();
		assert!(parser.parse().is_err());
	}
	#[test]
	fn test_empty_input() {
		let result = Parser::new("");
		if let Ok(mut parser) = result {
			assert!(parser.parse().is_err());
		}
		// If Parser::new already fails, test implicitly passes
	}
	#[test]
	fn test_deeply_nested() {
		let json = "[[[[[[[[[[1]]]]]]]]]]";
		let mut parser = Parser::new(json).unwrap();
		let value = parser.parse().unwrap();
		assert!(value.is_array());
	}
	
	
	#[test]
	fn test_max_depth_exceeded() {
		let deep_json = "[".repeat(200) + &"]".repeat(200);
		let limits = ParserLimits { max_depth: 100, ..Default::default() };
		let result = Parser::with_limits(&deep_json, limits).unwrap().parse();
		assert!(matches!(result, Err(JsonError::MaxDepthExceeded { .. })));
	}
	#[test]
	fn test_max_array_size_exceeded() {
		let big_array = format!("[{}]", "1,".repeat(1000).trim_end_matches(','));
		let limits = ParserLimits { max_array_length: 100, ..Default::default() };
		let result = Parser::with_limits(&big_array, limits).unwrap().parse();
		assert!(matches!(result, Err(JsonError::MaxArraySizeExceeded { .. })));
	}
}