use crate::JsonValue;
use std::fmt;
use std::fmt::{ Formatter};

#[derive(Debug, PartialEq, Clone)]
pub enum JsonNumber{
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

// ============================================================================
// JsonNumber - Conversion Methods & Trait Implementations
// ============================================================================

impl JsonNumber {
	// ========== Conversion Methods (as_*) ==========
	
	/// Convert to f64 (always succeeds)
	/// Used for numerical comparisons in validation
	#[inline]
	pub fn as_f64(&self) -> f64 {
		match self {
			JsonNumber::Integer(i) => *i as f64,
			JsonNumber::UnsignedInteger(u) => *u as f64,
			JsonNumber::Float(f) => *f,
		}
	}
	
	/// Try to convert to i64
	/// Returns None if value is unsigned and too large, or if float
	#[inline]
	pub fn as_i64(&self) -> Option<i64> {
		match self {
			JsonNumber::Integer(i) => Some(*i),
			JsonNumber::UnsignedInteger(u) => {
				if *u <= i64::MAX as u64 {
					Some(*u as i64)
				} else {
					None
				}
			},
			JsonNumber::Float(_) => None,
		}
	}
	
	/// Try to convert to u64
	/// Returns None if value is negative or float
	#[inline]
	pub fn as_u64(&self) -> Option<u64> {
		match self {
			JsonNumber::Integer(i) => {
				if *i >= 0 {
					Some(*i as u64)
				} else {
					None
				}
			},
			JsonNumber::UnsignedInteger(u) => Some(*u),
			JsonNumber::Float(_) => None,
		}
	}
	
	/// Try to convert to i32
	/// Returns None if value is out of i32 range or float
	#[inline]
	pub fn as_i32(&self) -> Option<i32> {
		match self {
			JsonNumber::Integer(i) => {
				if *i >= i32::MIN as i64 && *i <= i32::MAX as i64 {
					Some(*i as i32)
				} else {
					None
				}
			},
			JsonNumber::UnsignedInteger(u) => {
				if *u <= i32::MAX as u64 {
					Some(*u as i32)
				} else {
					None
				}
			},
			JsonNumber::Float(_) => None,
		}
	}
	
	/// Try to convert to u32
	/// Returns None if value is negative, out of range, or float
	#[inline]
	pub fn as_u32(&self) -> Option<u32> {
		match self {
			JsonNumber::Integer(i) => {
				if *i >= 0 && *i <= u32::MAX as i64 {
					Some(*i as u32)
				} else {
					None
				}
			},
			JsonNumber::UnsignedInteger(u) => {
				if *u <= u32::MAX as u64 {
					Some(*u as u32)
				} else {
					None
				}
			},
			JsonNumber::Float(_) => None,
		}
	}
	
	/// Try to convert to usize
	/// Returns None if value is negative, out of range, or float
	#[inline]
	pub fn as_usize(&self) -> Option<usize> {
		match self {
			JsonNumber::Integer(i) => {
				if *i >= 0 && *i <= usize::MAX as i64 {
					Some(*i as usize)
				} else {
					None
				}
			},
			JsonNumber::UnsignedInteger(u) => {
				if *u <= usize::MAX as u64 {
					Some(*u as usize)
				} else {
					None
				}
			},
			JsonNumber::Float(_) => None,
		}
	}
	
	/// Try to convert to isize
	/// Returns None if value is out of range or float
	#[inline]
	pub fn as_isize(&self) -> Option<isize> {
		match self {
			JsonNumber::Integer(i) => {
				if *i >= isize::MIN as i64 && *i <= isize::MAX as i64 {
					Some(*i as isize)
				} else {
					None
				}
			},
			JsonNumber::UnsignedInteger(u) => {
				if *u <= isize::MAX as u64 {
					Some(*u as isize)
				} else {
					None
				}
			},
			JsonNumber::Float(_) => None,
		}
	}
	
	/// Try to convert to f32
	/// Returns None if float is out of f32 range, or if integer too large
	#[inline]
	pub fn as_f32(&self) -> Option<f32> {
		let f64_val = self.as_f64();
		if f64_val.is_finite() && f64_val >= f32::MIN as f64 && f64_val <= f32::MAX as f64 {
			Some(f64_val as f32)
		} else {
			None
		}
	}
	
	// ========== Type Checking Methods ==========
	
	/// Check if this is an integer type (Integer or UnsignedInteger)
	#[inline]
	pub fn is_integer(&self) -> bool {
		matches!(self, JsonNumber::Integer(_) | JsonNumber::UnsignedInteger(_))
	}
	
	/// Check if this is a signed integer
	#[inline]
	pub fn is_i64(&self) -> bool {
		matches!(self, JsonNumber::Integer(_))
	}
	
	/// Check if this is an unsigned integer
	#[inline]
	pub fn is_u64(&self) -> bool {
		matches!(self, JsonNumber::UnsignedInteger(_))
	}
	
	/// Check if this is a float
	#[inline]
	pub fn is_float(&self) -> bool {
		matches!(self, JsonNumber::Float(_))
	}
	
	/// Check if the number is finite (not NaN or infinite)
	#[inline]
	pub fn is_finite(&self) -> bool {
		match self {
			JsonNumber::Integer(_) | JsonNumber::UnsignedInteger(_) => true,
			JsonNumber::Float(f) => f.is_finite(),
		}
	}
	
	/// Check if the number is NaN
	#[inline]
	pub fn is_nan(&self) -> bool {
		match self {
			JsonNumber::Float(f) => f.is_nan(),
			_ => false,
		}
	}
	
	/// Check if the number is infinite
	#[inline]
	pub fn is_infinite(&self) -> bool {
		match self {
			JsonNumber::Float(f) => f.is_infinite(),
			_ => false,
		}
	}
	
	/// Check if the number is positive
	#[inline]
	pub fn is_positive(&self) -> bool {
		match self {
			JsonNumber::Integer(i) => *i > 0,
			JsonNumber::UnsignedInteger(u) => *u > 0,
			JsonNumber::Float(f) => *f > 0.0,
		}
	}
	
	/// Check if the number is negative
	#[inline]
	pub fn is_negative(&self) -> bool {
		match self {
			JsonNumber::Integer(i) => *i < 0,
			JsonNumber::UnsignedInteger(_) => false,
			JsonNumber::Float(f) => *f < 0.0,
		}
	}
	
	/// Check if the number is zero
	#[inline]
	pub fn is_zero(&self) -> bool {
		match self {
			JsonNumber::Integer(i) => *i == 0,
			JsonNumber::UnsignedInteger(u) => *u == 0,
			JsonNumber::Float(f) => *f == 0.0,
		}
	}
	
	// ========== Equality Helpers ==========
	
	/// Check equality with epsilon for floats
	#[inline]
	pub fn eq_with_epsilon(&self, other: &JsonNumber, epsilon: f64) -> bool {
		(self.as_f64() - other.as_f64()).abs() < epsilon
	}
	
	/// Exact equality (for integers) or epsilon equality (for floats)
	#[inline]
	pub fn equals(&self, other: &JsonNumber) -> bool {
		match (self, other) {
			(JsonNumber::Integer(a), JsonNumber::Integer(b)) => a == b,
			(JsonNumber::UnsignedInteger(a), JsonNumber::UnsignedInteger(b)) => a == b,
			(JsonNumber::Integer(a), JsonNumber::UnsignedInteger(b)) => {
				*a >= 0 && *a as u64 == *b
			},
			(JsonNumber::UnsignedInteger(a), JsonNumber::Integer(b)) => {
				*b >= 0 && *a == *b as u64
			},
			_ => self.eq_with_epsilon(other, f64::EPSILON),
		}
	}

}

// ============================================================================
// Trait Implementations
// ============================================================================

// ========== PartialOrd for JsonNumber ==========

impl PartialOrd for JsonNumber {
	#[inline]
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		self.as_f64().partial_cmp(&other.as_f64())
	}
}

// ========== Compare JsonNumber with f64 ==========

impl PartialEq<f64> for JsonNumber {
	#[inline]
	fn eq(&self, other: &f64) -> bool {
		(self.as_f64() - other).abs() < f64::EPSILON
	}
}

impl PartialEq<JsonNumber> for f64 {
	#[inline]
	fn eq(&self, other: &JsonNumber) -> bool {
		(other.as_f64() - self).abs() < f64::EPSILON
	}
}

impl PartialOrd<f64> for JsonNumber {
	#[inline]
	fn partial_cmp(&self, other: &f64) -> Option<std::cmp::Ordering> {
		self.as_f64().partial_cmp(other)
	}
}

impl PartialOrd<JsonNumber> for f64 {
	#[inline]
	fn partial_cmp(&self, other: &JsonNumber) -> Option<std::cmp::Ordering> {
		self.partial_cmp(&other.as_f64())
	}
}

// ========== Compare JsonNumber with f32 ==========

impl PartialEq<f32> for JsonNumber {
	#[inline]
	fn eq(&self, other: &f32) -> bool {
		(self.as_f64() - (*other as f64)).abs() < f64::EPSILON
	}
}

impl PartialEq<JsonNumber> for f32 {
	#[inline]
	fn eq(&self, other: &JsonNumber) -> bool {
		(other.as_f64() - (*self as f64)).abs() < f64::EPSILON
	}
}

impl PartialOrd<f32> for JsonNumber {
	#[inline]
	fn partial_cmp(&self, other: &f32) -> Option<std::cmp::Ordering> {
		self.as_f64().partial_cmp(&(*other as f64))
	}
}

impl PartialOrd<JsonNumber> for f32 {
	#[inline]
	fn partial_cmp(&self, other: &JsonNumber) -> Option<std::cmp::Ordering> {
		(*self as f64).partial_cmp(&other.as_f64())
	}
}

// ========== Compare JsonNumber with i64 ==========

impl PartialEq<i64> for JsonNumber {
	#[inline]
	fn eq(&self, other: &i64) -> bool {
		match self {
			JsonNumber::Integer(i) => i == other,
			JsonNumber::UnsignedInteger(u) => *other >= 0 && *u == *other as u64,
			JsonNumber::Float(f) => (*f - (*other as f64)).abs() < f64::EPSILON,
		}
	}
}

impl PartialEq<JsonNumber> for i64 {
	#[inline]
	fn eq(&self, other: &JsonNumber) -> bool {
		other.eq(self)
	}
}

impl PartialOrd<i64> for JsonNumber {
	#[inline]
	fn partial_cmp(&self, other: &i64) -> Option<std::cmp::Ordering> {
		self.as_f64().partial_cmp(&(*other as f64))
	}
}

impl PartialOrd<JsonNumber> for i64 {
	#[inline]
	fn partial_cmp(&self, other: &JsonNumber) -> Option<std::cmp::Ordering> {
		(*self as f64).partial_cmp(&other.as_f64())
	}
}

// ========== Compare JsonNumber with u64 ==========

impl PartialEq<u64> for JsonNumber {
	#[inline]
	fn eq(&self, other: &u64) -> bool {
		match self {
			JsonNumber::Integer(i) => *i >= 0 && *i as u64 == *other,
			JsonNumber::UnsignedInteger(u) => u == other,
			JsonNumber::Float(f) => (*f - (*other as f64)).abs() < f64::EPSILON,
		}
	}
}

impl PartialEq<JsonNumber> for u64 {
	#[inline]
	fn eq(&self, other: &JsonNumber) -> bool {
		other.eq(self)
	}
}

impl PartialOrd<u64> for JsonNumber {
	#[inline]
	fn partial_cmp(&self, other: &u64) -> Option<std::cmp::Ordering> {
		self.as_f64().partial_cmp(&(*other as f64))
	}
}

impl PartialOrd<JsonNumber> for u64 {
	#[inline]
	fn partial_cmp(&self, other: &JsonNumber) -> Option<std::cmp::Ordering> {
		(*self as f64).partial_cmp(&other.as_f64())
	}
}

// ========== Compare JsonNumber with i32 ==========

impl PartialEq<i32> for JsonNumber {
	#[inline]
	fn eq(&self, other: &i32) -> bool {
		self.eq(&(*other as i64))
	}
}

impl PartialEq<JsonNumber> for i32 {
	#[inline]
	fn eq(&self, other: &JsonNumber) -> bool {
		other.eq(&(*self as i64))
	}
}

impl PartialOrd<i32> for JsonNumber {
	#[inline]
	fn partial_cmp(&self, other: &i32) -> Option<std::cmp::Ordering> {
		self.partial_cmp(&(*other as i64))
	}
}

impl PartialOrd<JsonNumber> for i32 {
	#[inline]
	fn partial_cmp(&self, other: &JsonNumber) -> Option<std::cmp::Ordering> {
		(*self as i64).partial_cmp(other)
	}
}

// ========== Compare JsonNumber with u32 ==========

impl PartialEq<u32> for JsonNumber {
	#[inline]
	fn eq(&self, other: &u32) -> bool {
		self.eq(&(*other as u64))
	}
}

impl PartialEq<JsonNumber> for u32 {
	#[inline]
	fn eq(&self, other: &JsonNumber) -> bool {
		other.eq(&(*self as u64))
	}
}

impl PartialOrd<u32> for JsonNumber {
	#[inline]
	fn partial_cmp(&self, other: &u32) -> Option<std::cmp::Ordering> {
		self.partial_cmp(&(*other as u64))
	}
}

impl PartialOrd<JsonNumber> for u32 {
	#[inline]
	fn partial_cmp(&self, other: &JsonNumber) -> Option<std::cmp::Ordering> {
		(*self as u64).partial_cmp(other)
	}
}

// ========== Compare JsonNumber with usize ==========

impl PartialEq<usize> for JsonNumber {
	#[inline]
	fn eq(&self, other: &usize) -> bool {
		self.eq(&(*other as u64))
	}
}

impl PartialEq<JsonNumber> for usize {
	#[inline]
	fn eq(&self, other: &JsonNumber) -> bool {
		other.eq(&(*self as u64))
	}
}

impl PartialOrd<usize> for JsonNumber {
	#[inline]
	fn partial_cmp(&self, other: &usize) -> Option<std::cmp::Ordering> {
		self.partial_cmp(&(*other as u64))
	}
}

impl PartialOrd<JsonNumber> for usize {
	#[inline]
	fn partial_cmp(&self, other: &JsonNumber) -> Option<std::cmp::Ordering> {
		(*self as u64).partial_cmp(other)
	}
}

// ========== Compare JsonNumber with isize ==========

impl PartialEq<isize> for JsonNumber {
	#[inline]
	fn eq(&self, other: &isize) -> bool {
		self.eq(&(*other as i64))
	}
}

impl PartialEq<JsonNumber> for isize {
	#[inline]
	fn eq(&self, other: &JsonNumber) -> bool {
		other.eq(&(*self as i64))
	}
}

impl PartialOrd<isize> for JsonNumber {
	#[inline]
	fn partial_cmp(&self, other: &isize) -> Option<std::cmp::Ordering> {
		self.partial_cmp(&(*other as i64))
	}
}

impl PartialOrd<JsonNumber> for isize {
	#[inline]
	fn partial_cmp(&self, other: &JsonNumber) -> Option<std::cmp::Ordering> {
		(*self as i64).partial_cmp(other)
	}
}
//
// // Example 1: Direct comparison with primitives
// fn validate_minimum(&self, value: &JsonNumber, min: f64) -> bool {
// 	value >= &min  // Direct comparison!
// }
//
// // Example 2: Clean conversion
// fn get_schema_version(obj: &[(String, JsonValue)]) -> Option<u32> {
// 	match get_field(obj, "version") {
// 		Some(JsonValue::Number(n)) => n.as_u32(),  // Clean!
// 		_ => None,
// 	}
// }
//
// // Example 3: Type checking
// fn validate_integer_type(value: &JsonValue) -> bool {
// 	match value {
// 		JsonValue::Number(n) => n.is_integer(),  // Clean!
// 		_ => false,
// 	}
// }
//
// // Example 4: Safe equality
// fn value_in_enum(value: &JsonNumber, allowed: &[JsonNumber]) -> bool {
// 	allowed.iter().any(|n| n.equals(value))  // Handles float epsilon!
// }
//
// // Example 5: Range checking
// fn in_range(value: &JsonNumber, min: f64, max: f64) -> bool {
// 	value >= &min && value <= &max  // Direct comparison!
// }