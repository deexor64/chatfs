use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub enum Command {
    #[serde(rename = "prompt")]
    Prompt,
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
