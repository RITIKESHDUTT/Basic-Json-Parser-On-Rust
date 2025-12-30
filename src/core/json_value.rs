use crate::JsonNumber;

#[derive(Debug, PartialEq, Clone, Eq)]
pub enum JsonValue {
    Null,
    Bool(bool),
    Number(JsonNumber),
    String(String),
    Array(Vec<JsonValue>),
    Object(Vec<(String, JsonValue)>),
}

// Compare JsonValue::String with &str
impl PartialEq<&str> for JsonValue {
    fn eq(&self, other: &&str) -> bool {
        match self {
            JsonValue::String(s) => s == *other,
            _ => false,
        }
    }
}

// Compare JsonValue::String with String
impl PartialEq<String> for JsonValue {
    fn eq(&self, other: &String) -> bool {
        match self {
            JsonValue::String(s) => s == other,
            _ => false,
        }
    }
}

// Compare JsonValue::Bool with bool
impl PartialEq<bool> for JsonValue {
    fn eq(&self, other: &bool) -> bool {
        match self {
            JsonValue::Bool(b) => b == other,
            _ => false,
        }
    }
}

// Compare JsonValue::Number with numeric primitives (delegate to JsonNumber)
impl PartialEq<f64> for JsonValue {
    fn eq(&self, other: &f64) -> bool {
        match self {
            JsonValue::Number(n) => n == other,
            _ => false,
        }
    }
}

impl PartialEq<i64> for JsonValue {
    fn eq(&self, other: &i64) -> bool {
        match self {
            JsonValue::Number(n) => n == other,
            _ => false,
        }
    }
}

impl PartialEq<u32> for JsonValue {
    fn eq(&self, other: &u32) -> bool {
        match self {
            JsonValue::Number(n) => n == other,
            _ => false,
        }
    }
}

impl JsonValue {
    pub fn is_null(&self) -> bool {
        matches!(self, JsonValue::Null)
    }
    
    pub fn is_bool(&self) -> bool {
        matches!(self, JsonValue::Bool(_))
    }
    
    pub fn is_number(&self) -> bool {
        matches!(self, JsonValue::Number(_))
    }
    
    pub fn is_string(&self) -> bool {
        matches!(self, JsonValue::String(_))
    }
    
    pub fn is_array(&self) -> bool {
        matches!(self, JsonValue::Array(_))
    }
    
    pub fn is_object(&self) -> bool {
        matches!(self, JsonValue::Object(_))
    }
    
    // Accessor methods with Options
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            JsonValue::Bool(b) => Some(*b),
            _ => None,
        }
    }
    
    pub fn as_str(&self) -> Option<&str> {
        match self {
            JsonValue::String(s) => Some(s),
            _ => None,
        }
    }
    
    pub fn as_number(&self) -> Option<&JsonNumber> {
        match self {
            JsonValue::Number(n) => Some(n),
            _ => None,
        }
    }
    
    pub fn as_array(&self) -> Option<&Vec<JsonValue>> {
        match self {
            JsonValue::Array(arr) => Some(arr),
            _ => None,
        }
    }
    
    pub fn as_object(&self) -> Option<&Vec<(String, JsonValue)>> {
        match self {
            JsonValue::Object(obj) => Some(obj),
            _ => None,
        }
    }
}

impl std::hash::Hash for JsonValue {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::mem::discriminant(self).hash(state);
        match self {
            JsonValue::Null => {},
            JsonValue::Bool(b) => b.hash(state),
            JsonValue::Number(n) => n.hash(state),
            JsonValue::String(s) => s.hash(state),
            JsonValue::Array(arr) => arr.hash(state),
            JsonValue::Object(obj) => obj.hash(state),
        }
    }
}