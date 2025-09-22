use crate::core::{JsonValue, JsonError};
use crate::engine::Parser;


pub fn deserialize(input: &str) -> Result<JsonValue, JsonError> {
    let mut parser = Parser::new(input)?;
    parser.parse()
}

pub fn serialize_pretty(value: &JsonValue) -> String {
    value.to_json_string_pretty()
}

pub fn serialize(value: &JsonValue) -> String {
    value.to_json_string()
}