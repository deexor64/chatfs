use std::collections::HashMap;
use serde_json::Value;

use super::types::Command;
use super::operations;


pub fn execute_command(command: &Command, queries: &HashMap<String, String>) -> Result<Value, String> {
    match command {
        Command::List => operations::list::list(queries),
        Command::Content => operations::content::content(queries),
        // Command::Create => operations::create::create(&queries),
        // Command::Copy => operations::copy::copy(&queries),
        // Command::Move => operations::mv::mv(&queries),
        // Command::Delete => operations::delete::delete(queries),
        // Command::Write => operations::write::write(queries)
        _ => Err("Disabled temporarily".to_string()),
    }
}
