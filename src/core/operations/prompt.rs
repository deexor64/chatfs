use serde_json::Value;

const PROMPT: &str = include_str!("../../../prompt.md");

pub fn prompt() -> Result<Value, String> {
    Ok(Value::String(PROMPT.to_string()))
}
