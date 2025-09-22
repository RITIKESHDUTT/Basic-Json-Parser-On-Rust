use crate::core::JsonNumber;
use crate::parser::NumberParser;

#[cfg(test)]
#[test]
fn test_display_integer() {
    let num = NumberParser::parse("42").unwrap();
    assert_eq!(format!("{}", num), "42");

    let neg = JsonNumber::Integer(-123);
    assert_eq!(format!("{}", neg), "-123");
}

#[test]
fn test_display_unsigned() {
    let num = JsonNumber::UnsignedInteger( u64::MAX);
    assert_eq!(format!("{}", num), "18446744073709551615");
}

#[test]
fn test_display_float() {
    let num = JsonNumber::Float(3.1415923);
    assert_eq!(format!("{}", num), "3.1415923");

    // Test whole number formatting
    let whole = JsonNumber::Float(42.0);
    assert_eq!(format!("{}", whole), "42");

    // Test scientific notation
    let sci = JsonNumber::Float(1.5e10);
    assert_eq!(format!("{}", sci), "15000000000");
}

#[test]
fn test_display_special_floats() {
    let nan = JsonNumber::Float(f64::NAN);
    assert_eq!(format!("{}", nan), "null");

    let inf = JsonNumber::Float(f64::INFINITY);
    assert_eq!(format!("{}", inf), "null");

    let neg_inf = JsonNumber::Float(f64::NEG_INFINITY);
    assert_eq!(format!("{}", neg_inf), "null");
}

#[test]
fn test_from_str_simple() {
    // Integer parsing
    let int_val = NumberParser::parse("42").unwrap();
    assert_eq!(int_val, JsonNumber::Integer(42));

    // Large positive integer -> UnsignedInteger
    let u_val = NumberParser::parse("18446744073709551615").unwrap();
    assert_eq!(u_val, JsonNumber::UnsignedInteger(18446744073709551615));

    // Float parsing
    let float_val = NumberParser::parse("3.14").unwrap();
    assert_eq!(float_val, JsonNumber::Float(3.14));

    // Scientific notation
    let sci_val = NumberParser::parse("1e10").unwrap();
    assert_eq!(sci_val, JsonNumber::Float(1e10));
}
