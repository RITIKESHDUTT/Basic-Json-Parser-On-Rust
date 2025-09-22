use crate::core::JsonNumber;

#[cfg(test)]
#[test]
fn test_display_integer() {
    let num = JsonNumber::Integer(42);
    assert_eq!(format!("{}", num), "42");

    let neg = JsonNumber::Integer(-123);
    assert_eq!(format!("{}", neg), "-123");
}

#[test]
fn test_display_unsigned() {
    let num = JsonNumber::UnsignedInteger(18446744073709551615); // u64::MAX
    assert_eq!(format!("{}", num), "18446744073709551615");
}

#[test]
fn test_display_float() {
    let num = JsonNumber::Float(3.14159);
    assert_eq!(format!("{}", num), "3.14159");

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
    assert_eq!(
        JsonNumber::from_str_simple("42").unwrap(),
        JsonNumber::Integer(42)
    );

    // Large positive integer -> UnsignedInteger
    assert_eq!(
        JsonNumber::from_str_simple("18446744073709551615").unwrap(),
        JsonNumber::UnsignedInteger(18446744073709551615)
    );

    // Float parsing
    assert_eq!(
        JsonNumber::from_str_simple("3.14").unwrap(),
        JsonNumber::Float(3.14)
    );

    // Scientific notation
    assert_eq!(
        JsonNumber::from_str_simple("1e10").unwrap(),
        JsonNumber::Float(1e10)
    );
}

#[test]
fn test_conversions() {
    let int_num = JsonNumber::Integer(42);
    assert_eq!(int_num.as_i64(), Some(42));
    assert_eq!(int_num.as_f64(), 42.0);
    assert!(int_num.is_integer());

    let float_int = JsonNumber::Float(42.0);
    assert_eq!(float_int.as_i64(), Some(42));
    assert!(float_int.is_integer());

    let float_decimal = JsonNumber::Float(3.14);
    assert_eq!(float_decimal.as_i64(), None);
    assert!(!float_decimal.is_integer());
}