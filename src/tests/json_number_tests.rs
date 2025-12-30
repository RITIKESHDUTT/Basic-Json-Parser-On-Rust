use crate::core::JsonNumber;


// ========== Conversion Tests ==========

#[test]
fn test_as_f64() {
    assert_eq!(JsonNumber::Integer(-42).as_f64(), -42.0);
    assert_eq!(JsonNumber::UnsignedInteger(100).as_f64(), 100.0);
    assert_eq!(JsonNumber::Float(3.14).as_f64(), 3.14);
}

#[test]
fn test_as_i64() {
    assert_eq!(JsonNumber::Integer(-42).as_i64(), Some(-42));
    assert_eq!(JsonNumber::UnsignedInteger(100).as_i64(), Some(100));
    assert_eq!(JsonNumber::UnsignedInteger(u64::MAX).as_i64(), None);
    assert_eq!(JsonNumber::Float(3.14).as_i64(), None);
}

#[test]
fn test_as_u64() {
    assert_eq!(JsonNumber::Integer(42).as_u64(), Some(42));
    assert_eq!(JsonNumber::Integer(-42).as_u64(), None);
    assert_eq!(JsonNumber::UnsignedInteger(100).as_u64(), Some(100));
    assert_eq!(JsonNumber::Float(3.14).as_u64(), None);
}

#[test]
fn test_as_i32() {
    assert_eq!(JsonNumber::Integer(42).as_i32(), Some(42));
    assert_eq!(JsonNumber::Integer(i64::MAX).as_i32(), None);
    assert_eq!(JsonNumber::UnsignedInteger(100).as_i32(), Some(100));
    assert_eq!(JsonNumber::Float(3.14).as_i32(), None);
}

#[test]
fn test_as_u32() {
    assert_eq!(JsonNumber::Integer(42).as_u32(), Some(42));
    assert_eq!(JsonNumber::Integer(-42).as_u32(), None);
    assert_eq!(JsonNumber::UnsignedInteger(100).as_u32(), Some(100));
    assert_eq!(JsonNumber::UnsignedInteger(u64::MAX).as_u32(), None);
}

#[test]
fn test_as_usize() {
    assert_eq!(JsonNumber::Integer(42).as_usize(), Some(42));
    assert_eq!(JsonNumber::Integer(-42).as_usize(), None);
    assert_eq!(JsonNumber::UnsignedInteger(100).as_usize(), Some(100));
    assert_eq!(JsonNumber::Float(3.14).as_usize(), None);
}



// ========== Type Checking Tests ==========

#[test]
fn test_is_integer() {
    assert!(JsonNumber::Integer(-42).is_integer());
    assert!(JsonNumber::UnsignedInteger(100).is_integer());
    assert!(!JsonNumber::Float(3.14).is_integer());
}

#[test]
fn test_is_float() {
    assert!(!JsonNumber::Integer(-42).is_float());
    assert!(!JsonNumber::UnsignedInteger(100).is_float());
    assert!(JsonNumber::Float(3.14).is_float());
}

#[test]
fn test_is_finite() {
    assert!(JsonNumber::Integer(42).is_finite());
    assert!(JsonNumber::Float(3.14).is_finite());
    assert!(!JsonNumber::Float(f64::NAN).is_finite());
    assert!(!JsonNumber::Float(f64::INFINITY).is_finite());
}

#[test]
fn test_is_positive() {
    assert!(JsonNumber::Integer(42).is_positive());
    assert!(!JsonNumber::Integer(-42).is_positive());
    assert!(JsonNumber::UnsignedInteger(100).is_positive());
    assert!(!JsonNumber::Integer(0).is_positive());
}

#[test]
fn test_is_negative() {
    assert!(JsonNumber::Integer(-42).is_negative());
    assert!(!JsonNumber::Integer(42).is_negative());
    assert!(!JsonNumber::UnsignedInteger(100).is_negative());
    assert!(JsonNumber::Float(-3.14).is_negative());
}

#[test]
fn test_is_zero() {
    assert!(JsonNumber::Integer(0).is_zero());
    assert!(JsonNumber::UnsignedInteger(0).is_zero());
    assert!(JsonNumber::Float(0.0).is_zero());
    assert!(!JsonNumber::Integer(42).is_zero());
}

// ========== PartialOrd Tests ==========

#[test]
fn test_partial_ord_numbers() {
    let a = JsonNumber::Integer(10);
    let b = JsonNumber::Float(10.5);
    let c = JsonNumber::UnsignedInteger(11);
    
    assert!(a < b);
    assert!(b < c);
    assert!(a < c);
    assert!(c > a);
}

// ========== PartialEq with Primitives Tests ==========

#[test]
fn test_eq_with_f64() {
    let num = JsonNumber::Float(3.14);
    assert!(num == 3.14);
    assert!(3.14 == num);
    assert!(num != 3.15);
}

#[test]
fn test_eq_with_i64() {
    let num = JsonNumber::Integer(42);
    assert!(num == 42i64);
    assert!(42i64 == num);
    assert!(num != 43i64);
}

#[test]
fn test_eq_with_u64() {
    let num = JsonNumber::UnsignedInteger(100);
    assert!(num == 100u64);
    assert!(100u64 == num);
    assert!(num != 101u64);
}

#[test]
fn test_cmp_with_primitives() {
    let num = JsonNumber::Integer(42);
    assert!(num < 50i64);
    assert!(num > 30i64);
    assert!(50i64 > num);
    assert!(30i64 < num);
}

// ========== Edge Cases ==========

#[test]
fn test_large_numbers() {
    let large_int = JsonNumber::Integer(i64::MAX);
    let large_uint = JsonNumber::UnsignedInteger(u64::MAX);
    
    assert!(large_int.as_i64().is_some());
    assert!(large_uint.as_i64().is_none());
    assert!(large_int.as_u64().is_some());
    assert!(large_uint.as_u64().is_some());
}

#[test]
fn test_mixed_type_comparison() {
    let int_num = JsonNumber::Integer(42);
    let uint_num = JsonNumber::UnsignedInteger(42);
    let float_num = JsonNumber::Float(42.0);
    
    assert!(int_num.equals(&uint_num));
    assert!(int_num.equals(&float_num));
    assert!(uint_num.equals(&float_num));
}

#[test]
fn test_negative_zero() {
    let pos_zero = JsonNumber::Float(0.0);
    let neg_zero = JsonNumber::Float(-0.0);
    
    assert!(pos_zero.is_zero());
    assert!(neg_zero.is_zero());
}