use std::path::PathBuf;
use serde::Deserialize;

// Commands
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

// Parameters
#[derive(Clone)]
pub struct OpPath {
    pub original: PathBuf,
    pub resolved: PathBuf,
    pub workspace: PathBuf,
}

#[derive(PartialEq, Clone)]
pub enum Line {
    Num(usize),
    All,
}

#[derive(PartialEq, Clone)]
pub enum ItemType {
    File, // imply exists
    Folder, // imply exists
    AnyExist,
    AnyNonExist
}

#[derive(PartialEq, Clone)]
pub enum WriteMode {
    Replace,
    Shift
}
