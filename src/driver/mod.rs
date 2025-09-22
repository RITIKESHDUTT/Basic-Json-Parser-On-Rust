use crate::engine::Parser;
use crate::core::{JsonValue};

pub fn serialize(value: &JsonValue) -> String {
    value.to_json_string_indent(0)
}

pub fn deserialize(input: &str) -> Result<JsonValue, Box<dyn std::error::Error>> {
    let mut parser = Parser::new(input);
    let value = parser.parse()?;
    Ok(value)
}