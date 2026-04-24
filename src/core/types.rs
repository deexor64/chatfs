use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize)]
pub enum Command {
    #[serde(rename = "list")]
    List,
    #[serde(rename = "content")]
    Content,
    #[serde(rename = "create")]
    Create,
    #[serde(rename = "copy")]
    Copy,
    #[serde(rename = "move")]
    Move,
    #[serde(rename = "delete")]
    Delete,
    #[serde(rename = "write")]
    Write
}

pub struct ExecutionResult {
    pub status: bool,
    pub result: Value,
}
