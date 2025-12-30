use crate::core::ParserLimits;
use crate::core::JsonError;
use crate::core::JsonValue;
use crate::engine::Parser;


pub fn deserialize(input: &str) -> Result<JsonValue, JsonError> {
    Parser::new(input)?.parse()
}
pub fn deserialize_with_limits(input: &str, limits: ParserLimits) -> Result<JsonValue, JsonError> {
    Parser::with_limits(input, limits)?.parse()
}
pub fn serialize_pretty(value: &JsonValue) -> String {
    value.to_json_string_pretty()
}

pub fn serialize(value: &JsonValue) -> String {
    value.to_json_string()
}