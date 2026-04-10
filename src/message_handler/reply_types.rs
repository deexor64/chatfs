use serde::Serialize;
use serde_json::Value;

// Reply field types
#[derive(Debug, Serialize)]
pub enum ReplyType {
    #[serde(rename = "code_context")]
    CodeContext,
}

// Acutal replies
#[derive(Serialize)]
pub struct CodeContext {
    pub id: String,
    pub status: bool,
    pub reply_type: ReplyType,
    pub context: Value,
}
