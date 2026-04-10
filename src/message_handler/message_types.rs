use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;

// Message field types
#[derive(Debug, Deserialize)]
pub enum MessageType {
    #[serde(rename = "connect_ack")]
    ConnectAck,
    #[serde(rename = "query_codebase")]
    QueryCodebase,
}

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

// Actual messages
#[derive(Debug, Deserialize)]
pub struct ConnectAck {
    pub status: bool,
    pub message_type: MessageType,
    pub server_url: String,
}

#[derive(Debug, Deserialize)]
pub struct QueryCodebase {
    pub id: String,
    pub status: bool,
    pub message_type: MessageType,
    pub command: Command,
    pub queries: HashMap<String, Value>
}
