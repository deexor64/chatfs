use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{collections::HashMap, fmt::{Display, Formatter}};

use crate::core::types::Command;

// Message types
#[derive(Debug, Deserialize)]
pub enum MessageType {
    #[serde(rename = "connect_syn")]
    ConnectSyn,
    #[serde(rename = "ping")]
    Ping,
    #[serde(rename = "llm_command")]
    LlmCommand,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ConnectSyn {
    pub status: bool,
    pub message_type: MessageType,
    pub gateway_url: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Ping {
    pub status: bool,
    pub message_type: MessageType,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LlmCommand {
    pub status: bool,
    pub message_type: MessageType,
    pub id: String,
    pub command: Command,
    pub params: HashMap<String, String>
}

// TODO: Add proper display implementation
impl Display for LlmCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {:?}", self.command, self.params)
    }
}

// Reply types
#[derive(Debug, Serialize)]
pub enum ReplyType {
    #[serde(rename = "connect_ack")]
    ConnectAck,
    #[serde(rename = "pong")]
    Pong,
    #[serde(rename = "llm_result")]
    LlmResult,
}

#[derive(Debug, Serialize)]
pub struct ConnectAck {
    pub status: bool,
    pub reply_type: ReplyType,
}

#[derive(Debug, Serialize)]
pub struct Pong {
    pub status: bool,
    pub reply_type: ReplyType,
}

#[derive(Debug, Serialize)]
pub struct LlmResult {
    pub status: bool,
    pub reply_type: ReplyType,
    pub id: String,
    pub result: Value
}
