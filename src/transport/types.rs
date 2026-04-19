pub mod message_types {
    use serde::Deserialize;
    use serde_json::Value;
    use std::collections::HashMap;

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
        pub status: bool,
        pub message_type: MessageType,
        pub id: String,
        pub command: Command,
        pub queries: HashMap<String, Value>
    }

}

pub mod reply_types {
    use serde::Serialize;
    use serde_json::Value;

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

}
