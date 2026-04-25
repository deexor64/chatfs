use serde_json::Value;

const PROMPT: &str = include_str!("../../../PROMPT.md");

pub fn prompt() -> Result<Value, String> {
    Ok(Value::String(PROMPT.to_string()))
}
