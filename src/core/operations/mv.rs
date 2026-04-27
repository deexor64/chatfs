use serde_json::{Value, json};
use std::{collections::HashMap, fs};

use super::copy::parse_queries;

// Function name is 'mv' because 'move' is a rust reserved keyword
pub fn mv(queries: &HashMap<String, String>) -> Result<Value, String> {
    let (source_path, dest_path) = parse_queries(queries)?;

    if dest_path.resolved.exists() && dest_path.resolved.is_file() && source_path.resolved.is_file() {
        fs::remove_file(&dest_path.resolved)
            .map_err(|e| format!("Failed to replace destination file '{}' ({})", dest_path.original.display(), e))?;
    }

    fs::rename(&source_path.resolved, &dest_path.resolved)
        .map_err(|e| format!("Failed to move '{}' to '{}' ({})", source_path.original.display(), dest_path.original.display(), e))?;

    Ok(json!({"message": format!("Moved '{}' to '{}'", source_path.original.display(), dest_path.original.display())}))
}
