use serde::Serialize;
use serde_json::Value;

// Reply field types
#[derive(Debug, Serialize)]
pub enum ReplyType {
    #[serde(rename = "message_error")]
    MessageError,
    #[serde(rename = "code_context")]
    CodeContext
}

#[derive(Serialize)]
pub struct MessageError {
    pub status: bool,
    pub reply_type: ReplyType,
    pub error: String
}

// Actual replies
#[derive(Serialize)]
pub struct CodeContext {
    pub status: bool,
    pub reply_type: ReplyType,
    pub id: String,
    pub context: Value
}
