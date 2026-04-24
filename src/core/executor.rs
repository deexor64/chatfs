use std::collections::HashMap;

use super::types::{Command, ExecutionResult};
// use crate::fs::operations;


pub fn execute_command(command: &Command, _queries: &HashMap<String, String>) -> Result<ExecutionResult, String> {
    match command {
        _ => Ok(ExecutionResult { status: true, result: serde_json::json!({"message": "Disabled temporarily"}) }),
        // Command::List => operations::list::list(queries),
        // Command::Content => operations::content::content(queries),
        // Command::Create => operations::create::create(&queries),
        // Command::Copy => operations::copy::copy(&queries),
        // Command::Move => operations::mv::mv(&queries),
        // Command::Delete => operations::delete::delete(queries),
        // Command::Write => operations::write::write(queries)
    }
}
