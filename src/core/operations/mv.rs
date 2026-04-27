use serde_json::{Value, json};
use std::{collections::HashMap, fs};

use super::copy::{parse_queries, resolve_destination};

// Function name is 'mv' because 'move' is a rust reserved keyword
pub fn mv(queries: &HashMap<String, String>) -> Result<Value, String> {
    let (source_path, dest_path) = validator(queries)?;
    let final_dest = resolve_destination(&source_path, dest_path)?;

    if final_dest.exists() && final_dest.is_file() && source_path.is_file() {
        fs::remove_file(&final_dest)
            .map_err(|e| format!("Failed to replace destination file '{}' ({})", final_dest.display(), e))?;
    }

    fs::rename(&source_path, &final_dest)
        .map_err(|e| format!("Failed to move '{}' to '{}' ({})", source_path.display(), final_dest.display(), e))?;

    Ok(json!({"message": format!("Moved '{}' to '{}'", source_path.display(), final_dest.display())}))
}
